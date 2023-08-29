use crate::{
    dialogs::editor::{DialogMsg, EditorData, EditorDialog, EditorOutput, EditorType},
    util::markdown_to_pango_markup,
};
use gtk::prelude::*;
use lemmy_api_common::{
    lemmy_db_schema::{SortType, SubscribedType},
    lemmy_db_views::structs::PostView,
    lemmy_db_views_actor::structs::CommunityView,
};
use relm4::{factory::FactoryVecDeque, prelude::*};
use relm4_components::web_image::WebImage;

use crate::{api, settings, util::get_web_image_msg};

use super::{
    post_row::PostRow,
    sort_dropdown::{SortDropdown, SortDropdownOutput},
};

pub struct CommunityPage {
    info: CommunityView,
    avatar: Controller<WebImage>,
    sort_dropdown: Controller<SortDropdown>,
    posts: FactoryVecDeque<PostRow>,
    #[allow(dead_code)]
    create_post_dialog: Controller<EditorDialog>,
    current_sort_type: SortType,
    current_posts_page: i64,
}

#[derive(Debug)]
pub enum CommunityInput {
    UpdateCommunity(CommunityView),
    FetchPosts,
    DoneFetchPosts(Vec<PostView>),
    OpenCreatePostDialog,
    CreatePostRequest(EditorData),
    CreatedPost(PostView),
    ToggleSubscription,
    UpdateSubscriptionState(SubscribedType),
    UpdateOrder(SortType),
    ToggleBlocked,
    UpdateBlocked(bool),
    None,
}

#[relm4::component(pub)]
impl SimpleComponent for CommunityPage {
    type Init = CommunityView;
    type Input = CommunityInput;
    type Output = crate::AppMsg;

    view! {
        gtk::ScrolledWindow {
            set_vexpand: false,
            connect_edge_reached[sender] => move |_,pos| {
                if pos == gtk::PositionType::Bottom {
                    sender.input(CommunityInput::FetchPosts)
                }
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_vexpand: false,
                set_margin_all: 10,

                #[local_ref]
                avatar -> gtk::Box {
                    set_size_request: (100, 100),
                    set_margin_bottom: 20,
                    set_margin_top: 20,
                },
                gtk::Label {
                    #[watch]
                    set_text: &model.info.community.name,
                    add_css_class: "font-very-bold",
                },
                gtk::Label {
                    #[watch]
                    set_markup: &markdown_to_pango_markup(model.info.community.description.clone().unwrap_or("".to_string())),
                    set_use_markup: true,
                    set_wrap: true,
                },
                gtk::Label {
                    #[watch]
                    set_text: &format!("{} subscribers, {} posts, {} comments", model.info.counts.subscribers, model.info.counts.posts, model.info.counts.comments),
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_margin_top: 10,
                    set_spacing: 10,
                    #[watch]
                    set_visible: settings::get_current_account().jwt.is_some(),

                    match model.info.subscribed {
                        SubscribedType::Subscribed => {
                            gtk::Button {
                                set_label: "Unsubscribe",
                                connect_clicked => CommunityInput::ToggleSubscription,
                            }
                        }
                        SubscribedType::NotSubscribed => {
                            gtk::Button {
                                set_label: "Subscribe",
                                connect_clicked => CommunityInput::ToggleSubscription,
                            }
                        }
                        SubscribedType::Pending => {
                            gtk::Label {
                                set_label: "Subscription pending",
                            }
                        }
                    },
                    gtk::Button {
                        set_label: "Block",
                        #[watch]
                        set_visible: !model.info.blocked,
                        connect_clicked => CommunityInput::ToggleBlocked,
                    },
                    gtk::Button {
                        set_label: "Unblock",
                        #[watch]
                        set_visible: model.info.blocked,
                        connect_clicked => CommunityInput::ToggleBlocked,
                    },
                    gtk::Button {
                        set_label: "Create post",
                        connect_clicked => CommunityInput::OpenCreatePostDialog,
                    }
                },

                #[local_ref]
                sort_dropdown -> gtk::DropDown {
                    set_margin_top: 10,
                    set_halign: gtk::Align::Start,
                },

                gtk::Separator {
                    set_margin_top: 10,
                },

                #[local_ref]
                posts -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                },

                gtk::Button {
                    set_label: "More",
                    set_margin_all: 10,
                    connect_clicked => CommunityInput::FetchPosts,
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let avatar = WebImage::builder().launch("".to_string()).detach();
        let posts = FactoryVecDeque::new(gtk::Box::default(), sender.output_sender());
        let sort_dropdown =
            SortDropdown::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    SortDropdownOutput::New(sort_order) => CommunityInput::UpdateOrder(sort_order),
                });

        let dialog = EditorDialog::builder()
            .transient_for(root)
            .launch(EditorType::Post)
            .forward(sender.input_sender(), |msg| match msg {
                EditorOutput::CreateRequest(post, _) => CommunityInput::CreatePostRequest(post),
                _ => CommunityInput::None,
            });

        let model = CommunityPage {
            info: init,
            avatar,
            sort_dropdown,
            posts,
            create_post_dialog: dialog,
            current_sort_type: SortType::Hot,
            current_posts_page: 0,
        };
        let avatar = model.avatar.widget();
        let sort_dropdown = model.sort_dropdown.widget();
        let posts = model.posts.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            CommunityInput::UpdateCommunity(community) => {
                self.info = community.clone();
                self.avatar
                    .emit(get_web_image_msg(community.community.icon));
                self.posts.guard().clear();
                self.current_posts_page = 0;
                if community.counts.posts == 0 {
                    return;
                }
                sender.input(CommunityInput::FetchPosts);
            }
            CommunityInput::FetchPosts => {
                let name = self.info.community.name.clone();
                let sort_type = self.current_sort_type;
                self.current_posts_page += 1;
                let page = self.current_posts_page;
                std::thread::spawn(move || {
                    let community_posts =
                        api::posts::list_posts(page, Some(name), None, Some(sort_type));
                    if let Ok(community_posts) = community_posts {
                        sender.input(CommunityInput::DoneFetchPosts(community_posts));
                    }
                });
            }
            CommunityInput::DoneFetchPosts(posts) => {
                for post in posts {
                    self.posts.guard().push_back(post);
                }
            }
            CommunityInput::OpenCreatePostDialog => {
                let sender = self.create_post_dialog.sender();
                sender.emit(DialogMsg::Show);
            }
            CommunityInput::CreatedPost(post) => {
                self.posts.guard().push_front(post);
            }
            CommunityInput::CreatePostRequest(post) => {
                let id = self.info.community.id.0;
                std::thread::spawn(move || {
                    let message = match api::post::create_post(post.name, post.body, post.url, id) {
                        Ok(post) => Some(CommunityInput::CreatedPost(post.post_view)),
                        Err(err) => {
                            println!("{}", err);
                            None
                        }
                    };
                    if let Some(message) = message {
                        sender.input(message)
                    };
                });
            }
            CommunityInput::ToggleSubscription => {
                let community_id = self.info.community.id;
                let new_state = matches!(self.info.subscribed, SubscribedType::NotSubscribed);
                std::thread::spawn(move || {
                    match api::community::follow_community(community_id, new_state) {
                        Ok(community) => {
                            sender.input(CommunityInput::UpdateSubscriptionState(
                                community.community_view.subscribed,
                            ));
                        }
                        Err(err) => {
                            println!("{}", err);
                        }
                    };
                });
            }
            CommunityInput::UpdateSubscriptionState(state) => {
                self.info.subscribed = state;
            }
            CommunityInput::UpdateOrder(sort_order) => {
                self.current_sort_type = sort_order;
                self.current_posts_page = 0;
                self.posts.guard().clear();
                sender.input_sender().emit(CommunityInput::FetchPosts);
            }
            CommunityInput::ToggleBlocked => {
                let community_id = self.info.community.id;
                let blocked = self.info.blocked;
                std::thread::spawn(move || {
                    match api::community::block_community(community_id, !blocked) {
                        Ok(resp) => {
                            sender.input(CommunityInput::UpdateBlocked(resp.blocked));
                        }
                        Err(err) => {
                            println!("{}", err);
                        }
                    }
                });
            }
            CommunityInput::UpdateBlocked(blocked) => self.info.blocked = blocked,
            CommunityInput::None => {}
        }
    }
}

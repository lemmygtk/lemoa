use crate::{
    dialogs::editor::{DialogMsg, EditorData, EditorDialog, EditorOutput, EditorType},
    util::markdown_to_pango_markup,
};
use gtk::prelude::*;
use lemmy_api_common::{
    lemmy_db_schema::SubscribedType, lemmy_db_views::structs::PostView,
    lemmy_db_views_actor::structs::CommunityView,
};
use relm4::{factory::FactoryVecDeque, prelude::*, MessageBroker};
use relm4_components::web_image::WebImage;

use crate::{api, settings, util::get_web_image_msg};

use super::post_row::PostRow;

static COMMUNITY_PAGE_BROKER: MessageBroker<DialogMsg> = MessageBroker::new();

pub struct CommunityPage {
    info: CommunityView,
    avatar: Controller<WebImage>,
    posts: FactoryVecDeque<PostRow>,
    #[allow(dead_code)]
    create_post_dialog: Controller<EditorDialog>,
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
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_margin_bottom: 10,

                    gtk::Label {
                        #[watch]
                        set_text: &format!("{} subscribers", model.info.counts.subscribers),
                        set_margin_end: 10,
                    },
                    gtk::Label {
                        #[watch]
                        set_text: &format!("{} posts, {} comments", model.info.counts.posts, model.info.counts.comments),
                        set_margin_start: 10,
                    },
                },

                if settings::get_current_account().jwt.is_some() {
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
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
                            set_label: "Create post",
                            set_margin_start: 10,
                            connect_clicked => CommunityInput::OpenCreatePostDialog,
                        }
                    }
                } else {
                    gtk::Box {}
                },

                gtk::Separator {},

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

        let dialog = EditorDialog::builder()
            .transient_for(root)
            .launch_with_broker(EditorType::Post, &COMMUNITY_PAGE_BROKER)
            .forward(sender.input_sender(), |msg| match msg {
                EditorOutput::CreateRequest(post, _) => CommunityInput::CreatePostRequest(post),
                _ => CommunityInput::None,
            });

        let model = CommunityPage {
            info: init,
            avatar,
            posts,
            create_post_dialog: dialog,
            current_posts_page: 0,
        };
        let avatar = model.avatar.widget();
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
                self.current_posts_page += 1;
                let page = self.current_posts_page;
                std::thread::spawn(move || {
                    let community_posts = api::posts::list_posts(page, Some(name), None);
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
            CommunityInput::OpenCreatePostDialog => COMMUNITY_PAGE_BROKER.send(DialogMsg::Show),
            CommunityInput::CreatedPost(post) => {
                self.posts.guard().push_front(post);
            }
            CommunityInput::CreatePostRequest(post) => {
                let id = self.info.community.id.0.clone();
                std::thread::spawn(move || {
                    let message = match api::post::create_post(post.name, post.body, post.url, id) {
                        Ok(post) => Some(CommunityInput::CreatedPost(post.post_view)),
                        Err(err) => {
                            println!("{}", err.to_string());
                            None
                        }
                    };
                    if let Some(message) = message {
                        sender.input(message)
                    };
                });
            }
            CommunityInput::ToggleSubscription => {
                let community_id = self.info.community.id.0;
                let new_state = match self.info.subscribed {
                    SubscribedType::NotSubscribed => true,
                    _ => false,
                };
                std::thread::spawn(move || {
                    let message = match api::community::follow_community(community_id, new_state) {
                        Ok(community) => Some(CommunityInput::UpdateSubscriptionState(
                            community.community_view.subscribed,
                        )),
                        Err(err) => {
                            println!("{}", err.to_string());
                            None
                        }
                    };
                    if message.is_some() {
                        sender.input(message.unwrap())
                    };
                });
            }
            CommunityInput::UpdateSubscriptionState(state) => {
                self.info.subscribed = state;
            }
            CommunityInput::None => {}
        }
    }
}

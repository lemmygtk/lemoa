use crate::{util::markdown_to_pango_markup, dialogs::create_post::{CreatePostDialog, CREATE_POST_DIALOG_BROKER, DialogMsg, CreatePostDialogOutput, DialogType}};
use lemmy_api_common::{lemmy_db_views::structs::PostView, lemmy_db_views_actor::structs::CommunityView, lemmy_db_schema::SubscribedType};
use relm4::{prelude::*, factory::FactoryVecDeque};
use gtk::prelude::*;
use relm4_components::web_image::WebImage;

use crate::{api, util::get_web_image_msg};

use super::post_row::PostRow;

pub struct CommunityPage {
    info: CommunityView,
    avatar: Controller<WebImage>,
    posts: FactoryVecDeque<PostRow>,
    #[allow(dead_code)]
    create_post_dialog: Controller<CreatePostDialog>,
}

#[derive(Debug)]
pub enum CommunityInput {
    UpdateCommunity(CommunityView),
    DoneFetchPosts(Vec<PostView>),
    OpenCreatePostDialog,
    CreatePostRequest(String, String),
    CreatedPost(PostView),
    ToggleSubscription,
    UpdateSubscriptionState(SubscribedType)
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
                    set_markup: &markdown_to_pango_markup(model.info.clone().community.description.unwrap_or("".to_string())),
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
                    gtk::Label {
                        #[watch]
                        set_text: &format!("{} posts, {} comments", model.info.counts.posts, model.info.counts.comments),
                        set_margin_start: 10,
                    },

                    gtk::Button {
                        set_label: "Create post",
                        set_margin_start: 10,
                        connect_clicked => CommunityInput::OpenCreatePostDialog,
                    }
                },

                gtk::Separator {},

                #[local_ref]
                posts -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
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

        let dialog = CreatePostDialog::builder()
            .transient_for(root)
            .launch_with_broker(DialogType::Post, &CREATE_POST_DIALOG_BROKER)
            .forward(sender.input_sender(),  |msg| match msg {
                CreatePostDialogOutput::CreateRequest(name, body) => CommunityInput::CreatePostRequest(name, body)
            });

        let model = CommunityPage { info: init, avatar, posts, create_post_dialog: dialog };
        let avatar = model.avatar.widget();
        let posts = model.posts.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            CommunityInput::UpdateCommunity(community) => {
                self.info = community.clone();
                self.avatar.emit(get_web_image_msg(community.community.icon));
                self.posts.guard().clear();

                std::thread::spawn(move || {
                    if community.counts.posts == 0 { return; }
                    let community_posts = api::posts::list_posts(1, Some(community.community.name), None);
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
                CREATE_POST_DIALOG_BROKER.send(DialogMsg::Show)
            }
            CommunityInput::CreatedPost(post) => {
                self.posts.guard().push_front(post);
            }
            CommunityInput::CreatePostRequest(name, body) => {
                let id = self.info.community.id.0.clone();
                std::thread::spawn(move || {
                    let message = match api::post::create_post(name, body, id) {
                        Ok(post) => Some(CommunityInput::CreatedPost(post.post_view)),
                        Err(err) => { println!("{}", err.to_string()); None }
                    };
                    if message.is_some() { sender.input(message.unwrap()) };
                });
            }
            CommunityInput::ToggleSubscription => {
                let community_id = self.info.community.id.0;
                let new_state = match self.info.subscribed {
                    SubscribedType::NotSubscribed => true,
                    _ => false
                };
                std::thread::spawn(move || {
                    let message = match api::community::follow_community(community_id, new_state) {
                        Ok(community) => Some(CommunityInput::UpdateSubscriptionState(community.community_view.subscribed)),
                        Err(err) => { println!("{}", err.to_string()); None }
                    };
                    if message.is_some() { sender.input(message.unwrap()) };
                });
            }
            CommunityInput::UpdateSubscriptionState(state) => {
                self.info.subscribed = state;
            }
        }
    }
}

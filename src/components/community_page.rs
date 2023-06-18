use lemmy_api_common::{community::GetCommunityResponse, lemmy_db_views::structs::PostView};
use relm4::{prelude::*, factory::FactoryVecDeque};
use gtk::prelude::*;
use relm4_components::web_image::WebImage;
use std::cell::RefCell;

use crate::{api, util::get_web_image_msg};

use super::post_row::PostRow;

pub struct CommunityPage {
    info: RefCell<GetCommunityResponse>,
    avatar: Controller<WebImage>,
    posts: FactoryVecDeque<PostRow>
}

#[derive(Debug)]
pub enum CommunityInput {
    UpdateCommunity(GetCommunityResponse),
    DoneFetchPosts(Vec<PostView>)
}

#[relm4::component(pub)]
impl SimpleComponent for CommunityPage {
    type Init = GetCommunityResponse;
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
                    set_text: &model.info.borrow().community_view.community.name,
                    add_css_class: "font-very-bold",
                },
                gtk::Label {
                    #[watch]
                    set_text: &model.info.borrow().clone().community_view.community.description.unwrap_or("".to_string()),
                },
                gtk::Label {
                    #[watch]
                    set_text: &format!("{} subscribers, ", model.info.borrow().community_view.counts.subscribers),
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_hexpand: false,
                    set_halign: gtk::Align::Center,

                    gtk::Label {
                        #[watch]
                        set_text: &format!("{} posts, ", model.info.borrow().community_view.counts.posts),
                    },
                    gtk::Label {
                        #[watch]
                        set_text: &format!("{} comments", model.info.borrow().clone().community_view.counts.comments),
                    },
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
        let model = CommunityPage { info: RefCell::new(init), avatar, posts };
        let avatar = model.avatar.widget();
        let posts = model.posts.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            CommunityInput::UpdateCommunity(community) => {
                *self.info.borrow_mut() = community.clone();
                self.avatar.emit(get_web_image_msg(community.community_view.community.icon));
                self.posts.guard().clear();

                std::thread::spawn(move || {
                    if community.community_view.counts.posts == 0 { return; }
                    let community_posts = api::posts::list_posts(1, Some(community.community_view.community.name));
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
        }
    }
}

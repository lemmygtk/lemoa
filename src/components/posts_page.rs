use gtk::prelude::*;
use lemmy_api_common::{lemmy_db_schema::ListingType, lemmy_db_views::structs::PostView};
use relm4::{factory::FactoryVecDeque, prelude::*};

use crate::api;

use super::post_row::PostRow;

pub struct PostsPage {
    posts: FactoryVecDeque<PostRow>,
    posts_type: ListingType,
    posts_page: i64,
}

#[derive(Debug)]
pub enum PostsPageInput {
    FetchPosts(ListingType, bool),
    DoneFetchPosts(Vec<PostView>),
}

#[relm4::component(pub)]
impl SimpleComponent for PostsPage {
    type Init = ();
    type Input = PostsPageInput;
    type Output = crate::AppMsg;

    view! {
        gtk::ScrolledWindow {
            set_hexpand: true,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 10,
                    set_margin_all: 10,

                    gtk::ToggleButton {
                        set_label: "All",
                        #[watch]
                        set_active: model.posts_type == ListingType::All,
                        connect_clicked => PostsPageInput::FetchPosts(ListingType::All, true),
                    },
                    gtk::ToggleButton {
                        set_label: "Local",
                        #[watch]
                        set_active: model.posts_type ==ListingType::Local,
                        connect_clicked => PostsPageInput::FetchPosts(ListingType::Local, true),
                    },
                    gtk::ToggleButton {
                        set_label: "Subscribed",
                        #[watch]
                        set_active: model.posts_type == ListingType::Subscribed,
                        connect_clicked => PostsPageInput::FetchPosts(ListingType::Subscribed, true),
                    }
                },
                #[local_ref]
                posts_box -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                },
                gtk::Button {
                    set_label: "More",
                    connect_clicked => PostsPageInput::FetchPosts(model.posts_type, false),
                    set_margin_all: 10,
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let posts = FactoryVecDeque::new(gtk::Box::default(), sender.output_sender());
        let model = Self {
            posts,
            posts_type: ListingType::Subscribed,
            posts_page: 1,
        };
        let posts_box = model.posts.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            PostsPageInput::FetchPosts(type_, remove_previous) => {
                self.posts_type = type_;
                let page = if remove_previous {
                    1
                } else {
                    self.posts_page + 1
                };
                // show the loading indicator if it's the first page
                if page == 1 {
                    sender
                        .output_sender()
                        .emit(crate::AppMsg::UpdateState(crate::AppState::Loading));
                }
                self.posts_page = page;
                std::thread::spawn(move || {
                    match api::posts::list_posts(page, None, Some(type_)) {
                        Ok(posts) => {
                            sender.input(PostsPageInput::DoneFetchPosts(posts));
                        }
                        Err(err) => sender
                            .output_sender()
                            .emit(crate::AppMsg::ShowMessage(err.to_string())),
                    };
                });
            }
            PostsPageInput::DoneFetchPosts(posts) => {
                sender
                    .output_sender()
                    .emit(crate::AppMsg::UpdateState(crate::AppState::Posts));
                if self.posts_page == 1 {
                    self.posts.guard().clear();
                }
                for post in posts {
                    self.posts.guard().push_back(post);
                }
            }
        }
    }
}

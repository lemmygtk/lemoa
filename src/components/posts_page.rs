use gtk::prelude::*;
use lemmy_api_common::{lemmy_db_schema::{ListingType, SortType}, lemmy_db_views::structs::PostView};
use relm4::{factory::FactoryVecDeque, prelude::*};

use crate::api;

use super::{post_row::PostRow, sort_dropdown::{SortDropdown, SortDropdownOutput}};

pub struct PostsPage {
    sort_dropdown: Controller<SortDropdown>,
    posts: FactoryVecDeque<PostRow>,
    posts_order: SortType,
    posts_type: ListingType,
    posts_page: i64,
}

#[derive(Debug)]
pub enum PostsPageInput {
    FetchPosts(ListingType, SortType, bool),
    DoneFetchPosts(Vec<PostView>),
    UpdateOrder(SortType),
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
                        connect_clicked => PostsPageInput::FetchPosts(ListingType::All, model.posts_order, true),
                    },
                    gtk::ToggleButton {
                        set_label: "Local",
                        #[watch]
                        set_active: model.posts_type ==ListingType::Local,
                        connect_clicked => PostsPageInput::FetchPosts(ListingType::Local, model.posts_order, true),
                    },
                    gtk::ToggleButton {
                        set_label: "Subscribed",
                        #[watch]
                        set_active: model.posts_type == ListingType::Subscribed,
                        connect_clicked => PostsPageInput::FetchPosts(ListingType::Subscribed, model.posts_order, true),
                    },

                    gtk::Box {
                        set_hexpand: true,
                    },

                    #[local_ref]
                    sort_dropdown -> gtk::DropDown {
                        set_margin_end: 10,
                    },
                },
                #[local_ref]
                posts_box -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                },
                gtk::Button {
                    set_label: "More",
                    connect_clicked => PostsPageInput::FetchPosts(model.posts_type, model.posts_order, false),
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
        let sort_dropdown = SortDropdown::builder().launch(()).forward(sender.input_sender(), |msg| {
            match msg {
                SortDropdownOutput::New(sort_order) => PostsPageInput::UpdateOrder(sort_order),
            }
        });
        let posts = FactoryVecDeque::new(gtk::Box::default(), sender.output_sender());
        let model = Self {
            sort_dropdown,
            posts,
            posts_type: ListingType::Local,
            posts_order: SortType::Hot,
            posts_page: 1,
        };
        let sort_dropdown = model.sort_dropdown.widget();
        let posts_box = model.posts.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            PostsPageInput::FetchPosts(type_, order, remove_previous) => {
                self.posts_type = type_;
                self.posts_order = order;
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
                    match api::posts::list_posts(page, None, Some(type_), Some(order)) {
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
            PostsPageInput::UpdateOrder(order) => {
                self.posts_order = order;
                sender.input_sender().emit(PostsPageInput::FetchPosts(self.posts_type, order, true));
            }
        }
    }
}

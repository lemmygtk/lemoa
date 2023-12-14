use gtk::prelude::*;
use lemmy_api_common::lemmy_db_views::structs::PostView;
use relm4::prelude::*;
use relm4_components::web_image::WebImage;

use crate::{api, util::get_web_image_url};
use crate::{settings, util};

use super::voting_row::{VotingRowModel, VotingStats};

#[derive(Debug)]
pub struct PostRow {
    post: PostView,
    author_image: Controller<WebImage>,
    community_image: Controller<WebImage>,
    thumbnail: Controller<WebImage>,
    voting_row: Controller<VotingRowModel>,
    image_size: i32,
}

#[derive(Debug)]
pub enum PostRowMsg {
    OpenPost,
    OpenCommunity,
    OpenPerson,
    ToggleSaved,
    ToggleRead,
    DeletePost,
    UpdateSaved(bool),
    UpdateRead(bool),
}

#[relm4::factory(pub)]
impl FactoryComponent for PostRow {
    type Init = PostView;
    type Input = PostRowMsg;
    type Output = crate::AppMsg;
    type CommandOutput = ();
    type ParentInput = crate::AppMsg;
    type ParentWidget = gtk::Box;

    view! {
        root = gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_margin_end: 10,
            set_margin_start: 10,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,

                #[local_ref]
                thumbnail -> gtk::Box {
                    set_visible: self.post.post.thumbnail_url.is_some(),
                    set_size_request: (self.image_size, self.image_size),
                    set_margin_start: 10,
                    set_margin_end: 10,
                    set_hexpand: false,
                    set_valign: gtk::Align::Center,
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Center,
                    set_spacing: 10,
                    set_hexpand: true,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_margin_top: 10,
                        set_spacing: 10,
                        set_vexpand: false,

                        #[local_ref]
                        community_image -> gtk::Box {
                            set_hexpand: false,
                            set_visible: self.post.community.icon.clone().is_some(),
                        },

                        gtk::Button {
                            set_label: &self.post.community.title,
                            connect_clicked => PostRowMsg::OpenCommunity,
                        },

                        #[local_ref]
                        author_image -> gtk::Box {
                            set_hexpand: false,
                            set_margin_start: 10,
                            set_visible: self.post.creator.avatar.clone().is_some(),
                        },

                        gtk::Button {
                            set_label: &self.post.creator.name,
                            connect_clicked => PostRowMsg::OpenPerson,
                        },

                        gtk::Label {
                            set_margin_start: 10,
                            set_label: &util::format_elapsed_time(self.post.post.published),
                        }
                    },

                    gtk::Label {
                        set_halign: gtk::Align::Start,
                        set_text: &self.post.post.name,
                        set_wrap: true,
                        add_css_class: "font-bold",
                        add_controller = gtk::GestureClick {
                            connect_pressed[sender] => move |_, _, _, _| {
                                sender.input(PostRowMsg::OpenPost);
                            }
                        },
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        #[local_ref]
                        voting_row -> gtk::Box {
                            set_margin_end: 10,
                        },
                        gtk::Label {
                            set_halign: gtk::Align::Start,
                            set_text: &format!("{} comments", self.post.counts.comments.clone()),
                        },
                        gtk::ToggleButton {
                            set_icon_name: "bookmark-new",
                            set_margin_start: 10,
                            connect_clicked => PostRowMsg::ToggleSaved,
                            set_visible: settings::get_current_account().jwt.is_some(),
                            #[watch]
                            set_active: self.post.saved,
                        },
                        gtk::ToggleButton {
                            set_icon_name: "mail-mark-read",
                            set_margin_start: 10,
                            connect_clicked => PostRowMsg::ToggleRead,
                            set_visible: settings::get_current_account().jwt.is_some(),
                            #[watch]
                            set_active: self.post.read,
                        },
                        gtk::Button {
                            set_icon_name: "edit-delete",
                            connect_clicked => PostRowMsg::DeletePost,
                            set_margin_start: 10,
                            #[watch]
                            set_visible: self.post.creator.id.0 == settings::get_current_account().id,
                        }
                    },
                }
            },

            gtk::Separator {
                set_margin_top: 10,
            }
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output)
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        let thumbnail = WebImage::builder()
            .launch(get_web_image_url(value.post.thumbnail_url.clone()))
            .detach();
        let author_image = WebImage::builder()
            .launch(get_web_image_url(value.creator.avatar.clone()))
            .detach();
        let community_image = WebImage::builder()
            .launch(get_web_image_url(value.community.icon.clone()))
            .detach();
        let voting_row = VotingRowModel::builder()
            .launch(VotingStats::from_post(value.counts.clone(), value.my_vote))
            .detach();

        Self {
            post: value,
            author_image,
            community_image,
            voting_row,
            thumbnail,
            image_size: 1500,
        }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        match root.widget_ref().toplevel_window() {
            Some(window) => {
                self.image_size = window.allocated_width() / 8;
            }
            None => unreachable!(),
        }
        let thumbnail = self.thumbnail.widget();
        let author_image = self.author_image.widget();
        let community_image = self.community_image.widget();
        let voting_row = self.voting_row.widget();
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            PostRowMsg::OpenCommunity => {
                sender.output(crate::AppMsg::OpenCommunity(self.post.community.id))
            }
            PostRowMsg::OpenPerson => {
                sender.output(crate::AppMsg::OpenPerson(self.post.creator.id))
            }
            PostRowMsg::OpenPost => sender.output(crate::AppMsg::OpenPost(self.post.post.id)),
            PostRowMsg::ToggleSaved => {
                let post_id = self.post.post.id;
                let new_state = !self.post.saved;
                std::thread::spawn(move || match api::post::save_post(post_id, new_state) {
                    Ok(_) => sender.input(PostRowMsg::UpdateSaved(new_state)),
                    Err(err) => println!("{}", err),
                });
            }
            PostRowMsg::ToggleRead => {
                let post_id = self.post.post.id;
                let new_state = !self.post.read;
                std::thread::spawn(move || {
                    match api::post::mark_post_as_read(post_id, new_state) {
                        Ok(_) => sender.input(PostRowMsg::UpdateRead(new_state)),
                        Err(err) => println!("{}", err),
                    }
                });
            }
            PostRowMsg::UpdateSaved(is_saved) => {
                self.post.saved = is_saved;
            }
            PostRowMsg::UpdateRead(is_read) => {
                self.post.read = is_read;
            }
            PostRowMsg::DeletePost => {
                let post_id = self.post.post.id;
                std::thread::spawn(move || {
                    let _ = api::post::delete_post(post_id);
                    sender.output_sender().emit(crate::AppMsg::OpenPosts);
                });
            }
        }
    }
}

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
    voting_row: Controller<VotingRowModel>,
}

#[derive(Debug)]
pub enum PostRowMsg {
    OpenPost,
    OpenCommunity,
    OpenPerson,
    DeletePost,
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
            set_spacing: 10,
            set_margin_end: 10,
            set_margin_start: 10,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_margin_top: 10,
                set_spacing: 10,
                set_vexpand: false,
                set_hexpand: true,

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
                gtk::Button {
                    set_icon_name: "edit-delete",
                    connect_clicked => PostRowMsg::DeletePost,
                    set_margin_start: 10,
                    #[watch]
                    set_visible: self.post.creator.id.0 == settings::get_current_account().id,
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
        }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let author_image = self.author_image.widget();
        let community_image = self.community_image.widget();
        let voting_row = self.voting_row.widget();
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            PostRowMsg::OpenCommunity => {
                sender.output(crate::AppMsg::OpenCommunity(self.post.community.id.clone()))
            }
            PostRowMsg::OpenPerson => {
                sender.output(crate::AppMsg::OpenPerson(self.post.creator.id.clone()))
            }
            PostRowMsg::OpenPost => {
                sender.output(crate::AppMsg::OpenPost(self.post.post.id.clone()))
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

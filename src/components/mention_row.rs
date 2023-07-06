use gtk::prelude::*;
use lemmy_api_common::lemmy_db_views_actor::structs::CommentReplyView;
use relm4::prelude::*;
use relm4_components::web_image::WebImage;

use crate::util;
use crate::util::get_web_image_url;
use crate::util::markdown_to_pango_markup;

use super::voting_row::VotingRowModel;
use super::voting_row::VotingStats;

#[derive(Debug)]
pub struct MentionRow {
    comment: CommentReplyView,
    creator_image: Controller<WebImage>,
    community_image: Controller<WebImage>,
    voting_row: Controller<VotingRowModel>,
}

#[derive(Debug)]
pub enum MentionRowMsg {
    OpenPerson,
    OpenPost,
    OpenCommunity,
}

#[relm4::factory(pub)]
impl FactoryComponent for MentionRow {
    type Init = CommentReplyView;
    type Input = MentionRowMsg;
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
            set_margin_top: 10,
            set_vexpand: false,

            gtk::Label {
                set_label: &self.comment.post.name,
                add_css_class: "font-bold",
                set_halign: gtk::Align::Start,
                set_wrap: true,
                add_controller = gtk::GestureClick {
                    connect_pressed[sender] => move |_, _, _, _| {
                        sender.input(MentionRowMsg::OpenCommunity);
                    }
                },
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                gtk::Label {
                    set_label: "in",
                },
                #[local_ref]
                community_image -> gtk::Box {
                    set_hexpand: false,
                    set_margin_start: 10,
                    set_margin_end: 10,
                    set_visible: self.comment.community.icon.clone().is_some(),
                },
                gtk::Button {
                    set_label: &self.comment.community.title,
                    connect_clicked => MentionRowMsg::OpenCommunity,
                },
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_margin_top: 10,
                set_vexpand: false,

                #[local_ref]
                creator_image -> gtk::Box {
                    set_hexpand: false,
                    set_visible: self.comment.creator.avatar.is_some(),
                },

                gtk::Button {
                    set_label: &self.comment.creator.name,
                    connect_clicked => MentionRowMsg::OpenPerson,
                },

                gtk::Label {
                    set_margin_start: 10,
                    set_label: &util::format_elapsed_time(self.comment.comment.published),
                }
            },

            gtk::Label {
                #[watch]
               set_markup: &markdown_to_pango_markup(self.comment.comment.content.clone()),
               set_wrap: true,
               set_halign: gtk::Align::Start,
               set_use_markup: true,
            },

            #[local_ref]
            voting_row -> gtk::Box {},

            gtk::Separator {}
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output)
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        let creator_image = WebImage::builder()
            .launch(get_web_image_url(value.creator.avatar.clone()))
            .detach();
        let community_image = WebImage::builder()
            .launch(get_web_image_url(value.community.icon.clone()))
            .detach();
        let voting_row = VotingRowModel::builder()
            .launch(VotingStats::from_comment(
                value.counts.clone(),
                value.my_vote,
            ))
            .detach();

        Self {
            comment: value,
            creator_image,
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
        let creator_image = self.creator_image.widget();
        let community_image = self.community_image.widget();
        let voting_row = self.voting_row.widget();
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            MentionRowMsg::OpenPerson => {
                sender.output(crate::AppMsg::OpenPerson(self.comment.creator.id.clone()));
            }
            MentionRowMsg::OpenPost => {
                sender.output(crate::AppMsg::OpenPost(self.comment.post.id.clone()));
            }
            MentionRowMsg::OpenCommunity => {
                sender.output(crate::AppMsg::OpenCommunity(
                    self.comment.community.id.clone(),
                ));
            }
        }
    }
}

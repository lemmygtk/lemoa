use gtk::prelude::*;
use lemmy_api_common::lemmy_db_schema::newtypes::{CommentId, PostId};
use relm4::prelude::*;

use crate::api;

pub struct ReportDialog {
    visible: bool,
    post_id: Option<PostId>,
    comment_id: Option<CommentId>,
}

#[derive(Debug)]
pub enum ReportDialogInput {
    Show,
    Hide,
    Report(String),
    UpdateId(Option<PostId>, Option<CommentId>),
}

#[relm4::component(pub)]
impl SimpleComponent for ReportDialog {
    type Init = (Option<PostId>, Option<CommentId>);
    type Input = ReportDialogInput;
    type Output = crate::AppMsg;

    view! {
        dialog = gtk::Dialog {
            #[watch]
            set_visible: model.visible,
            set_modal: true,
            set_title: Some("Report post/comment"),
            connect_close_request[sender] => move |_| {
                sender.input(ReportDialogInput::Hide);
                gtk::Inhibit(false)
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 15,

                #[name(report_message)]
                gtk::Entry {
                    set_placeholder_text: Some("Reason"),
                },

                gtk::Button {
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_label: "Report",
                    set_hexpand: false,
                    set_halign: gtk::Align::End,
                    connect_clicked[report_message] => move |_| {
                        let reason = report_message.text().to_string();
                        ReportDialogInput::Report(reason);
                    },
                },
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            visible: false,
            post_id: init.0,
            comment_id: init.1,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            ReportDialogInput::Show => {
                self.visible = true;
            }
            ReportDialogInput::Hide => {
                self.visible = false;
            }
            ReportDialogInput::Report(reason) => {
                let post_id = self.post_id;
                let comment_id = self.comment_id;

                std::thread::spawn(move || {
                    if let Some(post_id) = post_id {
                        _ = api::post::report_post(post_id, reason);
                    } else if let Some(comment_id) = comment_id {
                        _ = api::comment::report_comment(comment_id, reason);
                    }
                    sender.input_sender().emit(ReportDialogInput::Hide);
                });
            }
            ReportDialogInput::UpdateId(post_id, comment_id) => {
                self.post_id = post_id;
                self.comment_id = comment_id;
            }
        }
    }
}

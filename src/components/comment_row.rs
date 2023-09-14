use gtk::prelude::*;
use lemmy_api_common::lemmy_db_views::structs::CommentView;
use relm4::prelude::*;
use relm4_components::web_image::WebImage;

use crate::api;
use crate::dialogs::editor::DialogMsg;
use crate::dialogs::editor::EditorData;
use crate::dialogs::editor::EditorDialog;
use crate::dialogs::editor::EditorOutput;
use crate::dialogs::editor::EditorType;
use crate::dialogs::report_dialog::ReportDialog;
use crate::dialogs::report_dialog::ReportDialogInput;
use crate::settings;
use crate::util;
use crate::util::get_web_image_url;
use crate::util::markdown_to_pango_markup;

use super::voting_row::VotingRowModel;
use super::voting_row::VotingStats;

pub struct CommentRow {
    pub comment: CommentView,
    avatar: Controller<WebImage>,
    voting_row: Controller<VotingRowModel>,
    comment_editor_dialog: Controller<EditorDialog>,
    report_comment_dialog: Controller<ReportDialog>,
}

#[derive(Debug)]
pub enum CommentRowMsg {
    OpenPerson,
    DeleteComment,
    ToggleSaved,
    OpenEditor(bool),
    EditCommentRequest(EditorData),
    CreateCommentRequest(EditorData),
    UpdateComment(CommentView),
    ShowReportDialog,
}

#[relm4::factory(pub)]
impl FactoryComponent for CommentRow {
    type Init = CommentView;
    type Input = CommentRowMsg;
    type Output = crate::AppMsg;
    type CommandOutput = ();
    type ParentInput = crate::AppMsg;
    type ParentWidget = gtk::Box;

    view! {
        root = gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 10,
            set_margin_end: 10,
            set_margin_start: ((self.comment.comment.path.matches('.').count() - 1) * 20 + 10) as i32,
            set_margin_top: 10,

            gtk::Separator {},

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_vexpand: false,

                #[local_ref]
                community_image -> gtk::Box {
                    set_hexpand: false,
                    set_visible: self.comment.creator.avatar.is_some(),
                },

                gtk::Button {
                    set_label: &self.comment.creator.name,
                    connect_clicked => CommentRowMsg::OpenPerson,
                },

                gtk::Label {
                    set_margin_start: 10,
                    set_label: &util::format_elapsed_time(self.comment.comment.published),
                }
            },

            gtk::Label {
                #[watch]
               set_markup: &markdown_to_pango_markup(self.comment.comment.content.clone()),
               set_halign: gtk::Align::Start,
               set_wrap: true,
               set_use_markup: true,
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,

                #[local_ref]
                voting_row -> gtk::Box {},

                gtk::Button {
                    set_icon_name: "mail-replied",
                    connect_clicked => CommentRowMsg::OpenEditor(true),
                    #[watch]
                    set_visible: settings::get_current_account().jwt.is_some(),
                },

                gtk::ToggleButton {
                    set_icon_name: "bookmark-new",
                    set_margin_start: 5,
                    connect_clicked => CommentRowMsg::ToggleSaved,
                    set_visible: settings::get_current_account().jwt.is_some(),
                    #[watch]
                    set_active: self.comment.saved,
                },

                gtk::Button {
                    set_icon_name: "action-unavailable",
                    connect_clicked => CommentRowMsg::ShowReportDialog,
                    set_visible: settings::get_current_account().jwt.is_some(),
                },

                gtk::Button {
                    set_icon_name: "document-edit",
                    connect_clicked => CommentRowMsg::OpenEditor(false),
                    set_visible: self.comment.creator.id.0 == settings::get_current_account().id,
                },

                gtk::Button {
                    set_icon_name: "edit-delete",
                    connect_clicked => CommentRowMsg::DeleteComment,
                    set_visible: self.comment.creator.id.0 == settings::get_current_account().id,
                },
            },
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output)
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
        let avatar = WebImage::builder()
            .launch(get_web_image_url(value.creator.avatar.clone()))
            .detach();
        let voting_row = VotingRowModel::builder()
            .launch(VotingStats::from_comment(
                value.counts.clone(),
                value.my_vote,
            ))
            .detach();
        let comment_editor_dialog = EditorDialog::builder().launch(EditorType::Comment).forward(
            sender.input_sender(),
            |msg| match msg {
                EditorOutput::EditRequest(data, _) => CommentRowMsg::EditCommentRequest(data),
                EditorOutput::CreateRequest(data, _) => CommentRowMsg::CreateCommentRequest(data),
            },
        );
        let report_comment_dialog = ReportDialog::builder()
            .launch((None, Some(value.comment.id)))
            .detach();

        Self {
            comment: value,
            avatar,
            voting_row,
            comment_editor_dialog,
            report_comment_dialog,
        }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let community_image = self.avatar.widget();
        let voting_row = self.voting_row.widget();
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            CommentRowMsg::OpenPerson => {
                sender.output(crate::AppMsg::OpenPerson(self.comment.creator.id));
            }
            CommentRowMsg::DeleteComment => {
                let comment_id = self.comment.comment.id;
                std::thread::spawn(move || {
                    let _ = api::comment::delete_comment(comment_id);
                    sender.output_sender().emit(crate::AppMsg::OpenPosts);
                });
            }
            CommentRowMsg::OpenEditor(is_new) => {
                let data = if is_new {
                    EditorData::default()
                } else {
                    EditorData {
                        name: String::from(""),
                        body: self.comment.comment.content.clone(),
                        url: None,
                    }
                };
                let sender = self.comment_editor_dialog.sender();
                sender.emit(DialogMsg::UpdateData(data));
                sender.emit(DialogMsg::UpdateType(EditorType::Comment, is_new));
                sender.emit(DialogMsg::Show);
            }
            CommentRowMsg::UpdateComment(comment) => {
                self.comment = comment;
            }
            CommentRowMsg::EditCommentRequest(data) => {
                let id = self.comment.comment.id;
                std::thread::spawn(move || {
                    let message = match api::comment::edit_comment(data.body, id) {
                        Ok(comment) => Some(CommentRowMsg::UpdateComment(comment.comment_view)),
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
            CommentRowMsg::CreateCommentRequest(data) => {
                let post_id = self.comment.comment.post_id;
                let parent_id = self.comment.comment.id;
                std::thread::spawn(move || {
                    match api::comment::create_comment(post_id, data.body, Some(parent_id)) {
                        Ok(_comment) => {
                            // TODO sender.output_sender().emit(PostPageInput::CreatedComment(comment.comment_view));
                        }
                        Err(err) => {
                            println!("{}", err);
                        }
                    };
                });
            }
            CommentRowMsg::ToggleSaved => {
                let comment_id = self.comment.comment.id;
                let new_state = !self.comment.saved;
                std::thread::spawn(move || {
                    match api::comment::save_comment(comment_id, new_state) {
                        Ok(comment) => {
                            sender.input(CommentRowMsg::UpdateComment(comment.comment_view))
                        }
                        Err(err) => println!("{}", err),
                    }
                });
            }
            CommentRowMsg::ShowReportDialog => self
                .report_comment_dialog
                .sender()
                .emit(ReportDialogInput::Show),
        }
    }
}

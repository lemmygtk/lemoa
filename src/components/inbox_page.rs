use gtk::prelude::*;
use lemmy_api_common::lemmy_db_views_actor::structs::CommentReplyView;
use relm4::{factory::FactoryVecDeque, prelude::*};

use crate::api;

use super::mention_row::MentionRow;

#[derive(Debug, Clone)]
pub enum InboxType {
    Mentions,
    Replies,
}

pub struct InboxPage {
    mentions: FactoryVecDeque<MentionRow>,
    page: i64,
    unread_only: bool,
    type_: InboxType,
}

#[derive(Debug)]
pub enum InboxInput {
    UpdateType(InboxType),
    ToggleUnreadState,
    FetchInbox,
    UpdateInbox(Vec<CommentReplyView>),
    MarkAllAsRead,
}

#[relm4::component(pub)]
impl SimpleComponent for InboxPage {
    type Init = ();
    type Input = InboxInput;
    type Output = crate::AppMsg;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_margin_all: 15,
                set_spacing: 10,
                gtk::Button {
                    set_label: "Mentions",
                    connect_clicked => InboxInput::UpdateType(InboxType::Mentions),
                },
                gtk::Button {
                    set_label: "Replies",
                    connect_clicked => InboxInput::UpdateType(InboxType::Replies),
                },
                gtk::ToggleButton {
                    set_active: false,
                    set_label: "Show unread only",
                    connect_clicked => InboxInput::ToggleUnreadState,
                },
                gtk::Button {
                    set_label: "Mark all as read",
                    connect_clicked => InboxInput::MarkAllAsRead,
                }
            },
            gtk::ScrolledWindow {
                #[local_ref]
                mentions -> gtk::Box {
                    set_vexpand: true,
                    set_orientation: gtk::Orientation::Vertical,
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mentions = FactoryVecDeque::new(gtk::Box::default(), sender.output_sender());
        let model = Self {
            mentions,
            page: 1,
            unread_only: false,
            type_: InboxType::Mentions,
        };
        let mentions = model.mentions.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            InboxInput::FetchInbox => {
                let type_ = self.type_.clone();
                let page = self.page.clone();
                let unread_only = self.unread_only.clone();
                std::thread::spawn(move || {
                    let comments = match type_ {
                        InboxType::Mentions => {
                            if let Ok(response) = api::user::get_mentions(page, unread_only) {
                                // It's just a different object, but its contents are exactly the same
                                let serialised = serde_json::to_string(&response.mentions).unwrap();
                                serde_json::from_str(&serialised).ok()
                            } else {
                                None
                            }
                        }
                        InboxType::Replies => {
                            if let Ok(response) = api::user::get_replies(page, unread_only) {
                                Some(response.replies)
                            } else {
                                None
                            }
                        }
                    };
                    if let Some(comments) = comments {
                        sender.input(InboxInput::UpdateInbox(comments))
                    };
                });
            }
            InboxInput::UpdateType(type_) => {
                self.type_ = type_;
                sender.input(InboxInput::FetchInbox);
            }
            InboxInput::ToggleUnreadState => {
                self.unread_only = !self.unread_only;
                sender.input(InboxInput::FetchInbox);
            }
            InboxInput::UpdateInbox(comments) => {
                self.mentions.guard().clear();
                for comment in comments {
                    self.mentions.guard().push_back(comment);
                }
            }
            InboxInput::MarkAllAsRead => {
                let show_unread_only = self.unread_only.clone();
                std::thread::spawn(move || {
                    if api::user::mark_all_as_read().is_ok() && show_unread_only {
                        sender.input(InboxInput::UpdateInbox(vec![]));
                    }
                });
            }
        }
    }
}

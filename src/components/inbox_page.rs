use gtk::prelude::*;
use lemmy_api_common::{
    lemmy_db_views::structs::PrivateMessageView, lemmy_db_views_actor::structs::CommentReplyView,
};
use relm4::{factory::FactoryVecDeque, prelude::*};

use crate::api;

use super::{mention_row::MentionRow, private_message_row::PrivateMessageRow};

#[derive(Debug, Clone, PartialEq)]
pub enum InboxType {
    Replies,
    Mentions,
    PrivateMessages,
}

pub struct InboxPage {
    mentions: FactoryVecDeque<MentionRow>,
    private_messages: FactoryVecDeque<PrivateMessageRow>,
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
    UpdatePrivateMessages(Vec<PrivateMessageView>),
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
                gtk::ToggleButton {
                    set_label: "Replies",
                    connect_clicked => InboxInput::UpdateType(InboxType::Replies),
                    #[watch]
                    set_active: model.type_ == InboxType::Replies,
                },
                gtk::ToggleButton {
                    set_label: "Mentions",
                    connect_clicked => InboxInput::UpdateType(InboxType::Mentions),
                    #[watch]
                    set_active: model.type_ == InboxType::Mentions,
                },
                gtk::ToggleButton {
                    set_label: "Private messages",
                    connect_clicked => InboxInput::UpdateType(InboxType::PrivateMessages),
                    #[watch]
                    set_active: model.type_ == InboxType::PrivateMessages,
                },
                gtk::Box {
                    set_hexpand: true,
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
                match model.type_ {
                    InboxType::PrivateMessages => {
                        gtk::Box {
                            #[local_ref]
                            private_messages -> gtk::Box {
                                set_vexpand: true,
                                set_orientation: gtk::Orientation::Vertical,
                            }
                        }
                    }
                    _ => {
                        gtk::Box {
                            #[local_ref]
                            mentions -> gtk::Box {
                                set_vexpand: true,
                                set_orientation: gtk::Orientation::Vertical,
                            }
                        }
                    }
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
        let private_messages = FactoryVecDeque::new(gtk::Box::default(), sender.output_sender());
        let model = Self {
            mentions,
            private_messages,
            page: 1,
            unread_only: false,
            type_: InboxType::Replies,
        };
        let mentions = model.mentions.widget();
        let private_messages = model.private_messages.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            InboxInput::FetchInbox => {
                let type_ = self.type_.clone();
                let page = self.page;
                let unread_only = self.unread_only;
                std::thread::spawn(move || {
                    let message = match type_ {
                        InboxType::Mentions => {
                            if let Ok(response) = api::user::get_mentions(page, unread_only) {
                                // It's just a different object, but its contents are exactly the same
                                let serialised = serde_json::to_string(&response.mentions).unwrap();
                                let mentions = serde_json::from_str(&serialised).ok();
                                mentions.map(InboxInput::UpdateInbox)
                            } else {
                                None
                            }
                        }
                        InboxType::Replies => {
                            if let Ok(response) = api::user::get_replies(page, unread_only) {
                                Some(InboxInput::UpdateInbox(response.replies))
                            } else {
                                None
                            }
                        }
                        InboxType::PrivateMessages => {
                            if let Ok(response) =
                                api::private_message::list_private_messages(unread_only, page)
                            {
                                Some(InboxInput::UpdatePrivateMessages(response.private_messages))
                            } else {
                                None
                            }
                        }
                    };
                    if let Some(message) = message {
                        sender.input(message)
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
            InboxInput::UpdatePrivateMessages(messages) => {
                self.private_messages.guard().clear();
                for message in messages {
                    self.private_messages.guard().push_back(message);
                }
            }
            InboxInput::MarkAllAsRead => {
                let show_unread_only = self.unread_only;
                std::thread::spawn(move || {
                    if api::user::mark_all_as_read().is_ok() && show_unread_only {
                        sender.input(InboxInput::UpdateInbox(vec![]));
                    }
                });
            }
        }
    }
}

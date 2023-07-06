use gtk::prelude::*;
use relm4::prelude::*;

use crate::settings::{self, Account};

use super::accounts_page::AccountsPageInput;

pub struct AccountRow {
    account: Account,
    index: usize,
}

#[derive(Debug)]
pub enum AccountRowInput {
    Select,
    Logout,
    Delete,
}

#[relm4::factory(pub)]
impl FactoryComponent for AccountRow {
    type Init = Account;
    type Input = AccountRowInput;
    type Output = AccountsPageInput;
    type ParentInput = AccountsPageInput;
    type ParentWidget = gtk::Box;
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,

                gtk::Label {
                    #[watch]
                    set_label: &format!("{} ({})", self.account.name, self.account.instance_url),
                    add_controller = gtk::GestureClick {
                        connect_pressed[sender] => move |_, _, _, _| {
                            sender.input(AccountRowInput::Select);
                        }
                    },
                },
                gtk::Box {
                    set_hexpand: true,
                },
                gtk::Button {
                    set_icon_name: "system-log-out-symbolic",
                    connect_clicked => AccountRowInput::Logout,
                },
                gtk::Button {
                    set_icon_name: "edit-delete",
                    connect_clicked => AccountRowInput::Delete,
                    set_margin_start: 5,
                },
            },

            gtk::Separator {
                set_margin_all: 10,
            }
        }
    }

    fn init_model(init: Self::Init, index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        Self {
            account: init,
            index: index.current_index(),
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output)
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            AccountRowInput::Select => {
                settings::update_account_index(self.index);
                let message = if self.account.instance_url.is_empty() {
                    crate::AppMsg::ChooseInstance
                } else {
                    crate::AppMsg::OpenPosts
                };
                sender
                    .output_sender()
                    .emit(AccountsPageInput::Forward(message))
            }
            AccountRowInput::Logout => {
                self.account.name = "".to_string();
                self.account.id = 0;
                self.account.jwt = None;
                settings::update_account(self.account.clone(), self.index);
            }
            AccountRowInput::Delete => sender
                .output_sender()
                .emit(AccountsPageInput::Remove(self.index)),
        }
    }
}

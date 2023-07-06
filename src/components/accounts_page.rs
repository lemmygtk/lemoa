use gtk::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};

use crate::settings::{self, get_prefs, Account};

use super::account_row::AccountRow;

pub struct AccountsPage {
    accounts: FactoryVecDeque<AccountRow>,
}

#[derive(Debug)]
pub enum AccountsPageInput {
    Update,
    CreateNew,
    Remove(usize),
    Forward(crate::AppMsg),
}

#[relm4::component(pub)]
impl SimpleComponent for AccountsPage {
    type Init = ();
    type Input = AccountsPageInput;
    type Output = crate::AppMsg;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Label {
                set_label: "Accounts",
                add_css_class: "font-very-bold",
                set_margin_top: 10,
            },

            gtk::ScrolledWindow {
                set_vexpand: true,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_vexpand: true,
                    set_margin_all: 10,

                    #[local_ref]
                    accounts -> gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                    },

                    gtk::Button {
                        set_label: "New",
                        connect_clicked => AccountsPageInput::CreateNew,
                        set_margin_top: 10,
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
        let accounts = FactoryVecDeque::new(gtk::Box::builder().build(), sender.input_sender());
        sender.input(AccountsPageInput::Update);

        let model = Self { accounts };
        let accounts = model.accounts.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AccountsPageInput::Update => {
                self.accounts.guard().clear();
                for account in get_prefs().accounts {
                    self.accounts.guard().push_back(account);
                }
            }
            AccountsPageInput::CreateNew => {
                self.accounts.guard().push_back(Account::default());
                settings::create_account(false);
            }
            AccountsPageInput::Remove(index) => {
                if index as u32 != get_prefs().current_account_index {
                    self.accounts.guard().remove(index);
                    settings::remove_account(index);
                    // for now: update all to fix broken indexes
                    sender.input(AccountsPageInput::Update);
                }
            }
            AccountsPageInput::Forward(msg) => {
                sender.output_sender().emit(msg);
            }
        }
    }
}

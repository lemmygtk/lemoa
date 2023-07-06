use gtk::prelude::*;
use relm4::prelude::*;

use crate::{
    api,
    settings::{self, get_current_account},
};

pub struct LoginPage {}

#[derive(Debug)]
pub enum LoginPageInput {
    Login(String, String, String),
    Cancel,
}

#[relm4::component(pub)]
impl SimpleComponent for LoginPage {
    type Init = ();
    type Input = LoginPageInput;
    type Output = crate::AppMsg;

    view! {
        gtk::Box {
            set_hexpand: true,
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 12,
            set_margin_all: 20,
            set_valign: gtk::Align::Center,
            set_hexpand: true,

            gtk::Label {
                set_text: "Login",
                add_css_class: "font-bold",
            },
            #[name(username)]
            gtk::Entry {
                set_placeholder_text: Some("Username or E-Mail"),
            },
            #[name(password)]
            gtk::PasswordEntry {
                set_placeholder_text: Some("Password"),
                set_show_peek_icon: true,
            },
            #[name(totp_token)]
            gtk::Entry {
                set_placeholder_text: Some("Totp token (Optional)"),
            },
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_halign: gtk::Align::End,
                gtk::Button {
                    set_label: "Cancel",
                    connect_clicked => LoginPageInput::Cancel,
                    set_margin_end: 10,
                },
                gtk::Button {
                    set_label: "Login",
                    connect_clicked[sender, username, password, totp_token] => move |_| {
                        let username_text = username.text().as_str().to_string();
                        username.set_text("");
                        let password_text = password.text().as_str().to_string();
                        password.set_text("");
                        let totp_token_text = totp_token.text().as_str().to_string();
                        totp_token.set_text("");
                        sender.input(LoginPageInput::Login(username_text, password_text, totp_token_text));
                    },
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            LoginPageInput::Login(username, password, totp_token) => {
                if get_current_account().instance_url.is_empty() {
                    return;
                }
                let token = if totp_token.is_empty() {
                    None
                } else {
                    Some(totp_token)
                };
                sender
                    .output_sender()
                    .emit(crate::AppMsg::UpdateState(crate::AppState::Loading));

                std::thread::spawn(move || {
                    let message = match api::auth::login(username, password, token) {
                        Ok(login) => {
                            if let Some(token) = login.jwt {
                                let mut account = settings::get_current_account();
                                account.jwt = Some(token);
                                settings::update_current_account(account.clone());
                                if let Ok(site) = api::site::fetch_site() {
                                    let user = site.my_user.unwrap().local_user_view.person;
                                    account.name = user.name;
                                    account.id = user.id.0;
                                    settings::update_current_account(account);
                                }
                                crate::AppMsg::LoggedIn
                            } else {
                                crate::AppMsg::ShowMessage("Wrong credentials!".to_string())
                            }
                        }
                        Err(err) => crate::AppMsg::ShowMessage(err.to_string()),
                    };
                    sender.output_sender().emit(message);
                });
            }
            LoginPageInput::Cancel => {
                sender.output_sender().emit(crate::AppMsg::OpenPosts);
            }
        }
    }
}

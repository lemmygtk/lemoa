use crate::settings::{get_prefs, save_prefs};
use gtk::prelude::*;
use relm4::prelude::*;

pub struct Settings {
    visible: bool,
}

#[derive(Debug)]
pub enum SettingsInput {
    Show,
    Hide,
}

#[relm4::component(pub)]
impl SimpleComponent for Settings {
    type Init = ();
    type Input = SettingsInput;
    type Output = crate::AppMsg;

    view! {
        dialog = gtk::Dialog {
            #[watch]
            set_visible: model.visible,
            set_modal: true,
            set_title: Some("Settings"),
            connect_close_request[sender] => move |_| {
                sender.input(SettingsInput::Hide);
                gtk::Inhibit(false)
            },

            #[wrap(Some)]
            set_titlebar = &gtk::HeaderBar{
                set_show_title_buttons: true,
            },

            gtk::ScrolledWindow {
                set_size_request: (400, 400),

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                    set_margin_all: 15,

                    gtk::CheckButton {
                        set_active: get_prefs().infinite_scroll,
                        set_margin_all: 12,
                        set_label: Some("Infinite scroll"),
                        set_tooltip:"Fetch new content automatically when scrolling down",
                        connect_toggled => move |checkbox| {
                            let mut prefs = get_prefs();
                            prefs.infinite_scroll = checkbox.is_active();
                            save_prefs(&prefs);
                        },
                    },
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { visible: false };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            SettingsInput::Show => {
                self.visible = true;
            }
            SettingsInput::Hide => {
                self.visible = false;
            }
        }
    }
}

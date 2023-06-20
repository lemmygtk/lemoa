use relm4::{prelude::*, MessageBroker};
use gtk::prelude::*;

pub static CREATE_POST_DIALOG_BROKER: MessageBroker<DialogMsg> = MessageBroker::new();
pub static CREATE_COMMENT_DIALOG_BROKER: MessageBroker<DialogMsg> = MessageBroker::new();


pub struct CreatePostDialog {
    type_: DialogType,
    visible: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum DialogType {
    Post,
    Comment
}

#[derive(Debug)]
pub enum DialogMsg {
    Show,
    Hide,
    Okay(String, String)
}

#[derive(Debug)]
pub enum CreatePostDialogOutput {
    CreateRequest(String, String)
}

#[relm4::component(pub)]
impl SimpleComponent for CreatePostDialog {
    type Init = DialogType;
    type Input = DialogMsg;
    type Output = CreatePostDialogOutput;

    view! {
        dialog = gtk::Dialog {
            #[watch]
            set_visible: model.visible,
            set_modal: true,

            #[wrap(Some)]
            set_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_height_request: 400,
                set_width_request: 600,
                set_margin_all: 20,

                match model.type_ {
                    DialogType::Post => {
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            gtk::Label {
                                set_halign: gtk::Align::Center,
                                set_valign: gtk::Align::Center,
                                set_label: "Create post",
                                add_css_class: "font-bold"
                            },
                            #[name(name)]
                            gtk::Entry {    
                                set_placeholder_text: Some("Title"),
                                set_margin_top: 10,
                                set_margin_bottom: 10,
                            },
                        }
                    }
                    DialogType::Comment => {
                        gtk::Box {
                            gtk::Label {
                                set_halign: gtk::Align::Center,
                                set_valign: gtk::Align::Center,
                                set_label: "Create comment",
                                add_css_class: "font-bold"
                            },
                        }
                    }
                },
                gtk::Label {
                    set_text: "Body:",
                    set_halign: gtk::Align::Start,
                },
                #[name(body)]
                gtk::TextView {
                    set_editable: true,
                    set_margin_top: 5,
                    set_margin_bottom: 10,
                    set_vexpand: true,
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::End,
                    gtk::Button {
                        set_label: "Cancel",
                        set_margin_end: 10,
                        connect_clicked => DialogMsg::Hide,
                    },
                    gtk::Button {
                        set_label: "Okay",
                        connect_clicked[sender, name, body] => move |_| {
                            let name = name.text().to_string();
                            let body_buffer = body.buffer();
                            let (start, end) = &body_buffer.bounds();
                            let body = body_buffer.text(start, end, true).to_string();
                            match model.type_ {
                                DialogType::Post => {
                                    if name.is_empty() || body.is_empty() { return; }
                                    sender.input(DialogMsg::Okay(name, body))
                                }
                                DialogType::Comment => {
                                    if name.is_empty() { return; }
                                    sender.input(DialogMsg::Okay(name, body))
                                }
                            }
                        },
                    }
                }
            },

            connect_close_request[sender] => move |_| {
                sender.input(DialogMsg::Hide);
                gtk::Inhibit(false)
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = CreatePostDialog { type_: init, visible: false };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            DialogMsg::Show => self.visible = true,
            DialogMsg::Hide => self.visible = false,
            DialogMsg::Okay(name, body) => {
                let _ = sender.output(CreatePostDialogOutput::CreateRequest(name, body));
                self.visible = false;
            }
        }
    }
}
use relm4::prelude::*;
use gtk::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct EditorData {
    pub name: String,
    pub body: String,
    pub url: Option<reqwest::Url>,
    pub id: Option<i32>
}

pub struct EditorDialog {
    type_: EditorType,
    is_new: bool,
    visible: bool,
    name_buffer: gtk::EntryBuffer,
    url_buffer: gtk::EntryBuffer,
    body_buffer: gtk::TextBuffer,
    // Optional field to temporarily store the post or comment id
    id: Option<i32>
}

#[derive(Debug, Clone, Copy)]
pub enum EditorType {
    Post,
    Comment
}

#[derive(Debug)]
pub enum DialogMsg {
    Show,
    Hide,
    UpdateType(EditorType, bool),
    UpdateData(EditorData),
    Okay
}

#[derive(Debug)]
pub enum EditorOutput {
    CreateRequest(EditorData, EditorType),
    EditRequest(EditorData, EditorType)
}

#[relm4::component(pub)]
impl SimpleComponent for EditorDialog {
    type Init = EditorType;
    type Input = DialogMsg;
    type Output = EditorOutput;

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
                    EditorType::Post => {
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            gtk::Label {
                                set_halign: gtk::Align::Center,
                                set_valign: gtk::Align::Center,
                                set_label: "Post content",
                                add_css_class: "font-bold"
                            },
                            gtk::Entry {    
                                set_placeholder_text: Some("Title"),
                                set_margin_top: 10,
                                set_margin_bottom: 10,
                                set_buffer: &model.name_buffer,
                            },
                            gtk::Entry {    
                                set_placeholder_text: Some("Url"),
                                set_margin_top: 10,
                                set_margin_bottom: 10,
                                set_buffer: &model.url_buffer,
                            },
                        }
                    }
                    EditorType::Comment => {
                        gtk::Box {
                            gtk::Label {
                                set_halign: gtk::Align::Center,
                                set_valign: gtk::Align::Center,
                                set_label: "Comment content",
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
                        connect_clicked => DialogMsg::Okay,
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
        let name_buffer = gtk::EntryBuffer::builder().build();
        let url_buffer = gtk::EntryBuffer::builder().build();
        let body_buffer = gtk::TextBuffer::builder().build();
        let model = EditorDialog { type_: init, visible: false, is_new: true, name_buffer, url_buffer, body_buffer, id: None };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            DialogMsg::Show => self.visible = true,
            DialogMsg::Hide => {
                self.name_buffer.set_text("");
                self.url_buffer.set_text("");
                self.body_buffer.set_text("");
                self.visible = false;
            },
            DialogMsg::Okay => {
                let name = self.name_buffer.text().to_string();
                let url = self.url_buffer.text().to_string();
                let (start, end) = &self.body_buffer.bounds();
                let body = self.body_buffer.text(start, end, true).to_string();
                let url = reqwest::Url::parse(&url).ok();
                let post = EditorData { name, body, url, id: self.id };
                let message = match self.is_new {
                    true => EditorOutput::CreateRequest(post, self.type_),
                    false => EditorOutput::EditRequest(post, self.type_)
                };
                let _ = sender.output(message);
                self.visible = false;
            }
            DialogMsg::UpdateType(type_, is_new) => {
                self.type_ = type_;
                self.is_new = is_new;
            }
            DialogMsg::UpdateData(data) => {
                self.name_buffer.set_text(data.name);
                if let Some(url) = data.url { self.url_buffer.set_text(url.to_string()); }
                self.body_buffer.set_text(&data.body.clone());
            }
        }
    }
}
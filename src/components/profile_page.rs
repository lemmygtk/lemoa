use gtk::prelude::*;
use lemmy_api_common::person::GetPersonDetailsResponse;
use relm4::{factory::FactoryVecDeque, prelude::*};
use relm4_components::web_image::WebImage;

use crate::api;
use crate::dialogs::editor::DialogMsg;
use crate::dialogs::editor::EditorDialog;
use crate::dialogs::editor::EditorOutput;
use crate::dialogs::editor::EditorType;
use crate::settings;
use crate::util::get_web_image_msg;
use crate::util::markdown_to_pango_markup;

use super::post_row::PostRow;

pub struct ProfilePage {
    info: GetPersonDetailsResponse,
    avatar: Controller<WebImage>,
    posts: FactoryVecDeque<PostRow>,
    editor_dialog: Controller<EditorDialog>,
}

#[derive(Debug)]
pub enum ProfileInput {
    UpdatePerson(GetPersonDetailsResponse),
    SendMessageRequest,
    SendMessage(String),
}

#[relm4::component(pub)]
impl SimpleComponent for ProfilePage {
    type Init = GetPersonDetailsResponse;
    type Input = ProfileInput;
    type Output = crate::AppMsg;

    view! {
        gtk::ScrolledWindow {
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_vexpand: false,
                set_margin_all: 10,

                #[local_ref]
                avatar -> gtk::Box {
                    set_size_request: (100, 100),
                    set_margin_bottom: 20,
                    set_margin_top: 20,
                },
                gtk::Label {
                    #[watch]
                    set_text: &model.info.person_view.person.name,
                    add_css_class: "font-very-bold",
                },
                gtk::Label {
                    #[watch]
                    set_markup: &markdown_to_pango_markup(model.info.person_view.person.bio.clone().unwrap_or("".to_string())),
                    set_use_markup: true,
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_hexpand: false,
                    set_halign: gtk::Align::Center,

                    gtk::Label {
                        #[watch]
                        set_text: &format!("{} posts, {} comments", model.info.person_view.counts.post_count, model.info.person_view.counts.comment_count),
                    },
                    if settings::get_current_account().jwt.is_some() {
                        gtk::Button {
                            set_label: "Send message",
                            connect_clicked => ProfileInput::SendMessageRequest,
                            set_margin_start: 10,
                        }
                    } else {
                        gtk::Box {}
                    },
                },

                gtk::Separator {},

                #[local_ref]
                posts -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                }
            }

        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let avatar = WebImage::builder().launch("".to_string()).detach();
        let posts = FactoryVecDeque::new(gtk::Box::default(), sender.output_sender());
        let editor_dialog = EditorDialog::builder()
            .transient_for(root)
            .launch(EditorType::PrivateMessage)
            .forward(sender.input_sender(), |msg| match msg {
                EditorOutput::CreateRequest(data, _) => ProfileInput::SendMessage(data.body),
                _ => unreachable!(),
            });
        let model = ProfilePage {
            info: init,
            avatar,
            posts,
            editor_dialog,
        };
        let avatar = model.avatar.widget();
        let posts = model.posts.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            ProfileInput::UpdatePerson(person) => {
                self.info = person.clone();
                self.avatar
                    .emit(get_web_image_msg(person.person_view.person.avatar));
                self.posts.guard().clear();
                for post in person.posts {
                    self.posts.guard().push_back(post);
                }
            }
            ProfileInput::SendMessageRequest => self.editor_dialog.sender().emit(DialogMsg::Show),
            ProfileInput::SendMessage(content) => {
                let profile_id = self.info.person_view.person.id;
                std::thread::spawn(move || {
                    let _ = api::private_message::create_private_message(content, profile_id);
                });
            }
        }
    }
}

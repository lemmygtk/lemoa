use gtk::prelude::*;
use lemmy_api_common::{lemmy_db_schema::newtypes::PersonId, person::GetPersonDetailsResponse};
use relm4::{factory::FactoryVecDeque, prelude::*};
use relm4_components::web_image::WebImage;

use crate::api;
use crate::dialogs::editor::DialogMsg;
use crate::dialogs::editor::EditorDialog;
use crate::dialogs::editor::EditorOutput;
use crate::dialogs::editor::EditorType;
use crate::settings;
use crate::util::format_elapsed_time;
use crate::util::get_web_image_msg;
use crate::util::markdown_to_pango_markup;

use super::comment_row::CommentRow;
use super::moderates_row::ModeratesRow;
use super::post_row::PostRow;

pub struct ProfilePage {
    info: GetPersonDetailsResponse,
    avatar: Controller<WebImage>,
    posts: FactoryVecDeque<PostRow>,
    comments: FactoryVecDeque<CommentRow>,
    moderates: FactoryVecDeque<ModeratesRow>,
    editor_dialog: Controller<EditorDialog>,
    current_profile_page: i64,
    saved_only: bool,
}

#[derive(Debug)]
pub enum ProfileInput {
    FetchPerson(Option<PersonId>),
    UpdatePerson(GetPersonDetailsResponse, bool),
    BlockUser,
    SendMessageRequest,
    SendMessage(String),
}

#[relm4::component(pub)]
impl SimpleComponent for ProfilePage {
    type Init = (GetPersonDetailsResponse, bool);
    type Input = ProfileInput;
    type Output = crate::AppMsg;

    view! {
        gtk::ScrolledWindow {
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_vexpand: false,
                set_margin_all: 10,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_vexpand: false,
                    set_visible: !model.saved_only,

                    #[local_ref]
                    avatar -> gtk::Box {
                        set_size_request: (150, 150),
                        set_margin_bottom: 20,
                        set_margin_top: 20,
                        #[watch]
                        set_visible: model.info.person_view.person.avatar.is_some(),
                    },
                    gtk::Label {
                        #[watch]
                        set_text: &model.info.person_view.person.name,
                        add_css_class: "font-very-bold",
                    },
                    gtk::Label {
                        #[watch]
                        set_markup: &markdown_to_pango_markup(model.info.person_view.person.bio.clone().unwrap_or("".to_string())),
                        set_wrap: true,
                        set_use_markup: true,
                    },

                    gtk::Label {
                        set_margin_top: 5,
                        #[watch]
                        set_text: &format!("Joined {}", format_elapsed_time(model.info.person_view.person.published)),
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_margin_top: 10,
                        set_margin_bottom: 10,
                        set_spacing: 10,
                        set_hexpand: false,
                        set_halign: gtk::Align::Center,

                        gtk::Label {
                            #[watch]
                            set_text: &format!("{} posts, {} comments", model.info.person_view.counts.post_count, model.info.person_view.counts.comment_count),
                        },
                        gtk::Button {
                            set_label: "Block",
                            #[watch]
                            set_visible: settings::get_current_account().jwt.is_some() && settings::get_current_account().id != model.info.person_view.person.id.0,
                            connect_clicked => ProfileInput::BlockUser,
                        },
                        gtk::Button {
                            set_label: "Send message",
                            connect_clicked => ProfileInput::SendMessageRequest,
                            #[watch]
                            set_visible: settings::get_current_account().jwt.is_some(),
                        }
                    },

                    gtk::Separator {},
                },

                gtk::StackSwitcher {
                    set_stack: Some(&stack),
                },

                #[name(stack)]
                gtk::Stack {
                    add_child = &gtk::Box {
                        #[local_ref]
                        posts -> gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                        }
                    } -> {
                        set_title: "Posts",
                    },
                    add_child = &gtk::Box {
                        #[local_ref]
                        comments -> gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                        }
                    } -> {
                        set_title: "Comments",
                    },
                    add_child = &gtk::Box {
                        #[local_ref]
                        moderates -> gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                        }
                    } -> {
                        set_title: "Moderates",
                    },
                },

                gtk::Button {
                    set_label: "More",
                    set_margin_all: 10,
                    connect_clicked => ProfileInput::FetchPerson(None),
                }
            }

        }
    }

    fn init(
        (info, saved_only): Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let avatar = WebImage::builder().launch("".to_string()).detach();
        let posts = FactoryVecDeque::new(gtk::Box::default(), sender.output_sender());
        let moderates = FactoryVecDeque::new(gtk::Box::default(), sender.output_sender());
        let comments = FactoryVecDeque::new(gtk::Box::default(), sender.output_sender());
        let editor_dialog = EditorDialog::builder()
            .transient_for(root)
            .launch(EditorType::PrivateMessage)
            .forward(sender.input_sender(), |msg| match msg {
                EditorOutput::CreateRequest(data, _) => ProfileInput::SendMessage(data.body),
                _ => unreachable!(),
            });
        let model = ProfilePage {
            info,
            avatar,
            posts,
            comments,
            moderates,
            editor_dialog,
            current_profile_page: 1,
            saved_only,
        };
        let avatar = model.avatar.widget();
        let posts = model.posts.widget();
        let comments = model.comments.widget();
        let moderates = model.moderates.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            ProfileInput::UpdatePerson(person, clear) => {
                let state = if self.saved_only {
                    crate::AppState::Saved
                } else {
                    crate::AppState::Person
                };
                sender
                    .output_sender()
                    .emit(crate::AppMsg::UpdateState(state));

                if clear {
                    self.info = person.clone();
                    self.avatar
                        .emit(get_web_image_msg(person.person_view.person.avatar));

                    self.posts.guard().clear();
                    self.comments.guard().clear();
                    self.moderates.guard().clear();
                }

                for post in person.posts {
                    self.posts.guard().push_back(post);
                }
                for comment in person.comments {
                    self.comments.guard().push_back(comment);
                }
                for community in person.moderates {
                    self.moderates.guard().push_back(community);
                }
            }
            ProfileInput::SendMessageRequest => self.editor_dialog.sender().emit(DialogMsg::Show),
            ProfileInput::SendMessage(content) => {
                let profile_id = self.info.person_view.person.id;
                std::thread::spawn(move || {
                    let _ = api::private_message::create_private_message(content, profile_id);
                });
            }
            ProfileInput::FetchPerson(person_id) => {
                let page = if person_id.is_some() {
                    1
                } else {
                    self.current_profile_page + 1
                };
                self.current_profile_page = page;
                let person_id = person_id.unwrap_or(self.info.person_view.person.id);
                let saved_only = self.saved_only;
                std::thread::spawn(move || {
                    match api::user::get_user(person_id, page, saved_only) {
                        Ok(person) => {
                            sender.input(ProfileInput::UpdatePerson(person, page == 1));
                        }
                        Err(err) => {
                            sender
                                .output_sender()
                                .emit(crate::AppMsg::ShowMessage(err.to_string()));
                        }
                    };
                });
            }
            ProfileInput::BlockUser => {
                let person_id = self.info.person_view.person.id;
                std::thread::spawn(move || match api::user::block_user(person_id, true) {
                    Ok(_resp) => {}
                    Err(err) => {
                        println!("{}", err);
                    }
                });
            }
        }
    }
}

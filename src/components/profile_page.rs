use gtk::prelude::*;
use lemmy_api_common::{person::GetPersonDetailsResponse, lemmy_db_schema::newtypes::PersonId};
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
use super::community_row::CommunityRow;
use super::comment_row::CommentRow;

pub struct ProfilePage {
    info: GetPersonDetailsResponse,
    avatar: Controller<WebImage>,
    posts: FactoryVecDeque<PostRow>,
    comments: FactoryVecDeque<CommentRow>,
    communities: FactoryVecDeque<CommunityRow>,
    editor_dialog: Controller<EditorDialog>,
    current_profile_page: i64,
}

#[derive(Debug)]
pub enum ProfileInput {
    FetchPerson(Option<PersonId>),
    UpdatePerson(GetPersonDetailsResponse, bool),
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
                        communities -> gtk::Box {
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
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let avatar = WebImage::builder().launch("".to_string()).detach();
        let posts = FactoryVecDeque::new(gtk::Box::default(), sender.output_sender());
        let communities = FactoryVecDeque::new(gtk::Box::default(), sender.output_sender());
        let comments = FactoryVecDeque::new(gtk::Box::default(), sender.output_sender());
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
            comments,
            communities,
            editor_dialog,
            current_profile_page: 1,
        };
        let avatar = model.avatar.widget();
        let posts = model.posts.widget();
        let comments = model.comments.widget();
        let communities = model.communities.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            ProfileInput::UpdatePerson(person, clear) => {
                sender.output_sender().emit(crate::AppMsg::UpdateState(crate::AppState::Person));

                if clear {
                    self.info = person.clone();
                    self.avatar
                        .emit(get_web_image_msg(person.person_view.person.avatar));

                    self.posts.guard().clear();
                    self.comments.guard().clear();
                    self.communities.guard().clear();
                }

                for post in person.posts {
                    self.posts.guard().push_back(post);
                }
                for comment in person.comments {
                    self.comments.guard().push_back(comment);
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
                std::thread::spawn(move || {
                    match api::user::get_user(person_id, page) {
                        Ok(person) => {
                            sender.input(ProfileInput::UpdatePerson(person, page == 1));
                        },
                        Err(err) => {
                            sender.output_sender().emit(crate::AppMsg::ShowMessage(err.to_string()));
                        },
                    };
                });
            }
        }
    }
}

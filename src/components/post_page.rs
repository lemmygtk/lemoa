use gtk::prelude::*;
use lemmy_api_common::{
    lemmy_db_views::structs::{CommentView, PostView},
    post::GetPostResponse,
};
use relm4::{factory::FactoryVecDeque, prelude::*};
use relm4_components::web_image::WebImage;

use crate::{
    api,
    dialogs::editor::{DialogMsg, EditorData, EditorDialog, EditorOutput, EditorType},
    settings,
    util::{self, get_web_image_msg, get_web_image_url, markdown_to_pango_markup},
};

use super::{
    comment_row::CommentRow,
    voting_row::{VotingRowInput, VotingRowModel, VotingStats},
};

pub struct PostPage {
    info: GetPostResponse,
    image: Controller<WebImage>,
    creator_avatar: Controller<WebImage>,
    community_avatar: Controller<WebImage>,
    comments: FactoryVecDeque<CommentRow>,
    #[allow(dead_code)]
    create_comment_dialog: Controller<EditorDialog>,
    voting_row: Controller<VotingRowModel>,
}

#[derive(Debug)]
pub enum PostPageInput {
    UpdatePost(GetPostResponse),
    DoneFetchComments(Vec<CommentView>),
    OpenPerson,
    OpenCommunity,
    OpenLink,
    OpenImage,
    OpenCreateCommentDialog,
    CreateCommentRequest(EditorData),
    EditPostRequest(EditorData),
    CreatedComment(CommentView),
    OpenEditPostDialog,
    DeletePost,
    DoneEditPost(PostView),
    PassAppMessage(crate::AppMsg),
}

#[relm4::component(pub)]
impl SimpleComponent for PostPage {
    type Init = GetPostResponse;
    type Input = PostPageInput;
    type Output = crate::AppMsg;

    view! {
        gtk::ScrolledWindow {
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_vexpand: false,
                set_margin_all: 10,

                #[local_ref]
                image -> gtk::Box {
                    set_height_request: 400,
                    set_margin_bottom: 20,
                    set_margin_top: 20,
                    #[watch]
                    set_visible: model.info.post_view.post.thumbnail_url.is_some(),
                    add_controller = gtk::GestureClick {
                        connect_pressed[sender] => move |_, _, _, _| {
                            sender.input(PostPageInput::OpenImage);
                        }
                    },
                },
                gtk::Label {
                    #[watch]
                    set_text: &model.info.post_view.post.name,
                    set_wrap: true,
                    add_css_class: "font-very-bold",
                    add_controller = gtk::GestureClick {
                        connect_pressed[sender] => move |_, _, _, _| {
                            sender.input(PostPageInput::OpenLink);
                        }
                    },
                },
                gtk::Label {
                    #[watch]
                    set_markup: &markdown_to_pango_markup(model.info.post_view.post.body.clone().unwrap_or("".to_string())),
                    set_wrap: true,
                    set_margin_top: 10,
                    set_use_markup: true,
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_top: 10,
                    set_spacing: 10,
                    set_vexpand: false,
                    set_halign: gtk::Align::Center,

                    gtk::Label {
                        set_text: "posted by"
                    },

                    #[local_ref]
                    creator_avatar -> gtk::Box {
                        set_hexpand: false,
                        set_margin_start: 10,
                        set_visible: model.info.post_view.creator.avatar.is_some(),
                    },

                    gtk::Button {
                        #[watch]
                        set_label: &model.info.post_view.creator.name,
                        connect_clicked => PostPageInput::OpenPerson,
                    },

                    gtk::Label {
                        set_text: " in "
                    },

                    #[local_ref]
                    community_avatar -> gtk::Box {
                        set_hexpand: false,
                        set_visible: model.info.community_view.community.icon.is_some(),
                    },

                    gtk::Button {
                        #[watch]
                        set_label: &model.info.community_view.community.title,
                        connect_clicked => PostPageInput::OpenCommunity,
                    },

                    gtk::Button {
                        set_icon_name: "document-edit",
                        connect_clicked => PostPageInput::OpenEditPostDialog,
                        set_margin_start: 5,
                        #[watch]
                        set_visible: model.info.post_view.creator.id.0 == settings::get_current_account().id,
                    },

                    gtk::Button {
                        set_icon_name: "edit-delete",
                        connect_clicked => PostPageInput::DeletePost,
                        #[watch]
                        set_visible: model.info.post_view.creator.id.0 == settings::get_current_account().id,
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_top: 15,
                    set_margin_bottom: 10,
                    set_halign: gtk::Align::Center,

                    gtk::Label {
                        set_margin_end: 15,
                        set_label: &util::format_elapsed_time(model.info.post_view.post.published),
                    },
                    #[local_ref]
                    voting_row -> gtk::Box {
                        set_margin_end: 10,
                    },
                    gtk::Label {
                        #[watch]
                        set_text: &format!("{} comments", model.info.post_view.counts.comments),
                        set_margin_start: 10,
                    },
                    gtk::Button {
                        set_label: "Comment",
                        set_margin_start: 10,
                        connect_clicked => PostPageInput::OpenCreateCommentDialog,
                        #[watch]
                        set_visible: settings::get_current_account().jwt.is_some(),
                    }
                },

                gtk::Separator {},

                #[local_ref]
                comments -> gtk::Box {
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
        let image = WebImage::builder().launch("".to_string()).detach();
        let comments = FactoryVecDeque::new(gtk::Box::default(), sender.output_sender());
        let creator_avatar = WebImage::builder().launch("".to_string()).detach();
        let community_avatar = WebImage::builder().launch("".to_string()).detach();
        let dialog = EditorDialog::builder()
            .transient_for(root)
            .launch(EditorType::Comment)
            .forward(sender.input_sender(), |msg| match msg {
                EditorOutput::CreateRequest(comment, _) => {
                    PostPageInput::CreateCommentRequest(comment)
                }
                EditorOutput::EditRequest(post, _) => PostPageInput::EditPostRequest(post),
            });
        let voting_row = VotingRowModel::builder()
            .launch(VotingStats::default())
            .detach();
        let model = PostPage {
            info: init,
            image,
            comments,
            creator_avatar,
            community_avatar,
            create_comment_dialog: dialog,
            voting_row,
        };

        let image = model.image.widget();
        let comments = model.comments.widget();
        let creator_avatar = model.creator_avatar.widget();
        let community_avatar = model.community_avatar.widget();
        let voting_row = model.voting_row.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            PostPageInput::UpdatePost(post) => {
                self.info = post.clone();

                self.image
                    .emit(get_web_image_msg(post.post_view.post.thumbnail_url));
                self.community_avatar
                    .emit(get_web_image_msg(post.community_view.community.icon));
                self.creator_avatar
                    .emit(get_web_image_msg(post.post_view.creator.avatar));

                self.voting_row
                    .emit(VotingRowInput::UpdateStats(VotingStats::from_post(
                        post.post_view.counts.clone(),
                        post.post_view.my_vote,
                    )));
                self.comments.guard().clear();

                std::thread::spawn(move || {
                    if post.post_view.counts.comments == 0 {
                        return;
                    }
                    let comments = api::post::get_comments(post.post_view.post.id);
                    if let Ok(comments) = comments {
                        sender.input(PostPageInput::DoneFetchComments(comments));
                    }
                });
            }
            PostPageInput::DoneFetchComments(comments) => {
                for comment in comments {
                    self.comments.guard().push_back(comment);
                }
            }
            PostPageInput::OpenPerson => {
                let person_id = self.info.post_view.creator.id.clone();
                sender
                    .output_sender()
                    .emit(crate::AppMsg::OpenPerson(person_id));
            }
            PostPageInput::OpenCommunity => {
                let community_id = self.info.community_view.community.id.clone();
                sender
                    .output_sender()
                    .emit(crate::AppMsg::OpenCommunity(community_id));
            }
            PostPageInput::OpenLink => {
                let post = self.info.post_view.post.clone();
                let mut link = get_web_image_url(post.url);
                if link.is_empty() {
                    link = get_web_image_url(post.embed_video_url);
                }
                if !link.is_empty() {
                    gtk::show_uri(None::<&relm4::gtk::Window>, &link, 0);
                }
            }
            PostPageInput::OpenImage => {
                let link = get_web_image_url(self.info.post_view.post.thumbnail_url.clone());
                if !link.is_empty() {
                    gtk::show_uri(None::<&relm4::gtk::Window>, &link, 0);
                }
            }
            PostPageInput::OpenCreateCommentDialog => {
                let sender = self.create_comment_dialog.sender();
                sender.emit(DialogMsg::UpdateType(EditorType::Comment, true));
                sender.emit(DialogMsg::Show);
            }
            PostPageInput::CreatedComment(comment) => {
                self.comments.guard().push_front(comment);
            }
            PostPageInput::CreateCommentRequest(post) => {
                let id = self.info.post_view.post.id;
                std::thread::spawn(move || {
                    let message = match api::comment::create_comment(id, post.body, None) {
                        Ok(comment) => Some(PostPageInput::CreatedComment(comment.comment_view)),
                        Err(err) => {
                            println!("{}", err.to_string());
                            None
                        }
                    };
                    if let Some(message) = message {
                        sender.input(message)
                    };
                });
            }
            PostPageInput::DeletePost => {
                let post_id = self.info.post_view.post.id;
                std::thread::spawn(move || {
                    let _ = api::post::delete_post(post_id);
                    sender
                        .output_sender()
                        .emit(crate::AppMsg::StartFetchPosts(None, true));
                });
            }
            PostPageInput::OpenEditPostDialog => {
                let url = match self.info.post_view.post.url.clone() {
                    Some(url) => url.to_string(),
                    None => String::from(""),
                };
                let data = EditorData {
                    name: self.info.post_view.post.name.clone(),
                    body: self
                        .info
                        .post_view
                        .post
                        .body
                        .clone()
                        .unwrap_or(String::from("")),
                    url: reqwest::Url::parse(&url).ok(),
                };
                let sender = self.create_comment_dialog.sender();
                sender.emit(DialogMsg::UpdateData(data));
                sender.emit(DialogMsg::UpdateType(EditorType::Post, false));
                sender.emit(DialogMsg::Show);
            }
            PostPageInput::EditPostRequest(post) => {
                let id = self.info.post_view.post.id.0;
                std::thread::spawn(move || {
                    let message = match api::post::edit_post(post.name, post.url, post.body, id) {
                        Ok(post) => Some(PostPageInput::DoneEditPost(post.post_view)),
                        Err(err) => {
                            println!("{}", err.to_string());
                            None
                        }
                    };
                    if let Some(message) = message {
                        sender.input(message)
                    };
                });
            }
            PostPageInput::DoneEditPost(post) => {
                self.info.post_view = post;
            }
            PostPageInput::PassAppMessage(message) => {
                sender.output_sender().emit(message);
            }
        }
    }
}

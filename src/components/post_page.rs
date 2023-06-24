use lemmy_api_common::{lemmy_db_views::structs::{CommentView, PostView}, post::GetPostResponse};
use relm4::{prelude::*, factory::FactoryVecDeque, MessageBroker};
use gtk::prelude::*;
use relm4_components::web_image::WebImage;

use crate::{api, util::{get_web_image_msg, get_web_image_url, markdown_to_pango_markup}, dialogs::editor::{EditorDialog, EditorOutput, DialogMsg, EditorType, EditorData}, settings};

use super::{comment_row::CommentRow, voting_row::{VotingRowModel, VotingStats, VotingRowInput}};

pub static POST_PAGE_BROKER: MessageBroker<DialogMsg> = MessageBroker::new();

pub struct PostPage {
    info: GetPostResponse,
    image: Controller<WebImage>,
    creator_avatar: Controller<WebImage>,
    community_avatar: Controller<WebImage>,
    comments: FactoryVecDeque<CommentRow>,
    #[allow(dead_code)]
    create_comment_dialog: Controller<EditorDialog>,
    voting_row: Controller<VotingRowModel>
}

#[derive(Debug)]
pub enum PostInput {
    UpdatePost(GetPostResponse),
    DoneFetchComments(Vec<CommentView>),
    OpenPerson,
    OpenCommunity,
    OpenLink,
    OpenCreateCommentDialog,
    CreateCommentRequest(EditorData),
    EditPostRequest(EditorData),
    CreatedComment(CommentView),
    OpenEditPostDialog,
    OpenEditCommentDialog(EditorData),
    DeletePost,
    DoneEditPost(PostView),
    PassAppMessage(crate::AppMsg),
    EditCommentRequest(EditorData),
    UpdateComment(CommentView),
}

#[relm4::component(pub)]
impl SimpleComponent for PostPage {
    type Init = GetPostResponse;
    type Input = PostInput;
    type Output = crate::AppMsg;

    view! {
        gtk::ScrolledWindow {
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_vexpand: false,
                set_margin_all: 10,
            
                #[local_ref]
                image -> gtk::Box {
                    set_height_request: 100,
                    set_margin_bottom: 20,
                    set_margin_top: 20,
                },
                gtk::Label {
                    #[watch]
                    set_text: &model.info.post_view.post.name,
                    add_css_class: "font-very-bold",
                },
                gtk::Label {
                    #[watch]
                    set_markup: &markdown_to_pango_markup(model.info.post_view.post.body.clone().unwrap_or("".to_string())),
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

                    if model.info.post_view.creator.avatar.is_some() {
                        gtk::Box {
                            set_hexpand: false,
                            set_margin_start: 10,
                            #[local_ref]
                            creator_avatar -> gtk::Box {}
                        }
                    } else {
                        gtk::Box {}
                    },

                    gtk::Button {
                        set_label: &model.info.post_view.creator.name,
                        connect_clicked => PostInput::OpenPerson,
                    },

                    gtk::Label {
                        set_text: " in "
                    },

                    if model.info.community_view.community.icon.is_some() {
                        gtk::Box {
                            set_hexpand: false,
                            #[local_ref]
                            community_avatar -> gtk::Box {}
                        }
                    } else {
                        gtk::Box {}
                    },

                    gtk::Button {
                        set_label: &model.info.community_view.community.title,
                        connect_clicked => PostInput::OpenCommunity,
                    },

                    gtk::Box {
                        set_hexpand: true,
                    },

                    gtk::Button {
                        set_label: "View",
                        connect_clicked => PostInput::OpenLink,
                    },

                    if model.info.post_view.creator.id.0 == settings::get_current_account().id {
                        gtk::Button {
                            set_icon_name: "document-edit",
                            connect_clicked => PostInput::OpenEditPostDialog,
                            set_margin_start: 5,
                        }
                    } else {
                        gtk::Box {}
                    },

                    if model.info.post_view.creator.id.0 == settings::get_current_account().id {
                        gtk::Button {
                            set_icon_name: "edit-delete",
                            connect_clicked => PostInput::DeletePost,
                            set_margin_start: 5,
                        }
                    } else {
                        gtk::Box {}
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_halign: gtk::Align::Center,

                    #[local_ref]
                    voting_row -> gtk::Box {
                        set_margin_end: 10,
                    },
                    gtk::Label {
                        #[watch]
                        set_text: &format!("{} comments", model.info.post_view.counts.comments),
                    },
                    if settings::get_current_account().jwt.is_some() {
                        gtk::Button {
                            set_label: "Comment",
                            set_margin_start: 10,
                            connect_clicked => PostInput::OpenCreateCommentDialog,
                        }
                    } else {
                        gtk::Box {}
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
        let comments = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        let creator_avatar = WebImage::builder().launch("".to_string()).detach();
        let community_avatar = WebImage::builder().launch("".to_string()).detach();
        let dialog = EditorDialog::builder()
            .transient_for(root)
            .launch_with_broker(EditorType::Comment, &POST_PAGE_BROKER)
            .forward(sender.input_sender(),  |msg| match msg {
                EditorOutput::CreateRequest(comment, _) => PostInput::CreateCommentRequest(comment),
                EditorOutput::EditRequest(post, type_) => match type_ {
                    EditorType::Post => PostInput::EditPostRequest(post),
                    EditorType::Comment => PostInput::EditCommentRequest(post)
                }
            });
        let voting_row = VotingRowModel::builder().launch(VotingStats::default()).detach();
        let model = PostPage { info: init, image, comments, creator_avatar, community_avatar, create_comment_dialog: dialog, voting_row };
        
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
            PostInput::UpdatePost(post) => {
                self.info = post.clone();
                
                self.image.emit(get_web_image_msg(post.post_view.post.thumbnail_url));
                self.community_avatar.emit(get_web_image_msg(post.community_view.community.icon));
                self.creator_avatar.emit(get_web_image_msg(post.post_view.creator.avatar));

                self.voting_row.emit(VotingRowInput::UpdateStats(VotingStats::from_post(post.post_view.counts.clone(), post.post_view.my_vote)));
                self.comments.guard().clear();

                std::thread::spawn(move || {
                    if post.post_view.counts.comments == 0 { return; }
                    let comments = api::post::get_comments(post.post_view.post.id);
                    if let Ok(comments) = comments {
                        sender.input(PostInput::DoneFetchComments(comments));
                    }
                });
            }
            PostInput::DoneFetchComments(comments) => {
                for comment in comments {
                    self.comments.guard().push_back(comment);
                }
            }
            PostInput::OpenPerson => {
                let person_id = self.info.post_view.creator.id.clone();
                let _ = sender.output(crate::AppMsg::OpenPerson(person_id));
            }
            PostInput::OpenCommunity => {
                let community_id = self.info.community_view.community.id.clone();
                let _ = sender.output(crate::AppMsg::OpenCommunity(community_id));
            }
            PostInput::OpenLink => {
                let post = self.info.post_view.post.clone();
                let mut link = get_web_image_url(post.url);
                if link.is_empty() {
                    link = get_web_image_url(post.thumbnail_url);
                }
                if link.is_empty() {
                    link = get_web_image_url(post.embed_video_url);
                }
                if link.is_empty() { return; }
                gtk::show_uri(None::<&relm4::gtk::Window>, &link, 0);
            }
            PostInput::OpenCreateCommentDialog => {
                POST_PAGE_BROKER.send(DialogMsg::UpdateType(EditorType::Comment, true));
                POST_PAGE_BROKER.send(DialogMsg::Show);
            }
            PostInput::CreatedComment(comment) => {
                self.comments.guard().push_front(comment);
            }
            PostInput::CreateCommentRequest(post) => {
                let id = self.info.post_view.post.id.0;
                std::thread::spawn(move || {
                    let message = match api::comment::create_comment(id, post.body, None) {
                        Ok(comment) => Some(PostInput::CreatedComment(comment.comment_view)),
                        Err(err) => { println!("{}", err.to_string()); None }
                    };
                    if let Some(message) = message { sender.input(message) };
                });
            }
            PostInput::DeletePost => {
                let post_id = self.info.post_view.post.id;
                std::thread::spawn(move || {
                    let _ = api::post::delete_post(post_id);
                    let _ = sender.output(crate::AppMsg::StartFetchPosts(None, true));
                });
            }
            PostInput::OpenEditPostDialog => {
                let url = match self.info.post_view.post.url.clone() {
                    Some(url) => url.to_string(),
                    None => String::from("")
                };
                let data = EditorData {
                    name: self.info.post_view.post.name.clone(),
                    body: self.info.post_view.post.body.clone().unwrap_or(String::from("")),
                    url: reqwest::Url::parse(&url).ok(),
                    id: None,
                };
                POST_PAGE_BROKER.send(DialogMsg::UpdateData(data));
                POST_PAGE_BROKER.send(DialogMsg::UpdateType(EditorType::Post, false));
                POST_PAGE_BROKER.send(DialogMsg::Show)
            }
            PostInput::EditPostRequest(post) => {
                let id = self.info.post_view.post.id.0;
                std::thread::spawn(move || {
                    let message = match api::post::edit_post(post.name, post.url, post.body, id) {
                        Ok(post) => Some(PostInput::DoneEditPost(post.post_view)),
                        Err(err) => { println!("{}", err.to_string()); None }
                    };
                    if let Some(message) = message { sender.input(message) };
                });
            }
            PostInput::DoneEditPost(post) => {
                self.info.post_view = post;
            }
            PostInput::OpenEditCommentDialog(data) => {
                POST_PAGE_BROKER.send(DialogMsg::UpdateData(data));
                POST_PAGE_BROKER.send(DialogMsg::UpdateType(EditorType::Comment, false));
                POST_PAGE_BROKER.send(DialogMsg::Show);
            }
            PostInput::EditCommentRequest(data) => {
                std::thread::spawn(move || {
                    let message = match api::comment::edit_comment(data.body, data.id.unwrap()) {
                        Ok(comment) => Some(PostInput::UpdateComment(comment.comment_view)),
                        Err(err) => { println!("{}", err.to_string()); None }
                    };
                    if let Some(message) = message { sender.input(message) };
                });
            }
            PostInput::UpdateComment(comment) => {
                let mut index = 0;
                let id = comment.comment.id;
                loop {
                    if self.comments.guard().get(index).unwrap().comment.comment.id == id {
                        self.comments.guard().get_mut(index).unwrap().comment = comment;
                        break;
                    }
                    index += 1;
                }
            }
            PostInput::PassAppMessage(message) => {
                let _ = sender.output(message);
            }
        }
    }
}

use lemmy_api_common::{lemmy_db_views::structs::CommentView, post::GetPostResponse};
use relm4::{prelude::*, factory::FactoryVecDeque};
use gtk::prelude::*;
use relm4_components::web_image::WebImage;

use crate::{api, util::{get_web_image_msg, get_web_image_url, markdown_to_pango_markup}, dialogs::create_post::{CreatePostDialog, CreatePostDialogOutput, DialogMsg, CREATE_COMMENT_DIALOG_BROKER, DialogType}};

use super::comment_row::CommentRow;

pub struct PostPage {
    info: GetPostResponse,
    image: Controller<WebImage>,
    creator_avatar: Controller<WebImage>,
    community_avatar: Controller<WebImage>,
    comments: FactoryVecDeque<CommentRow>,
    #[allow(dead_code)]
    create_comment_dialog: Controller<CreatePostDialog>
}

#[derive(Debug)]
pub enum PostInput {
    UpdatePost(GetPostResponse),
    DoneFetchComments(Vec<CommentView>),
    OpenPerson,
    OpenCommunity,
    OpenLink,
    OpenCreateCommentDialog,
    CreateCommentRequest(String),
    CreatedComment(CommentView)
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
                        set_text: "posted by "
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
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_halign: gtk::Align::Center,

                    gtk::Label {
                        #[watch]
                        set_text: &format!("{} comments, {} score", model.info.post_view.counts.comments, model.info.post_view.counts.score),
                    },
                    gtk::Button {
                        set_label: "Comment",
                        set_margin_start: 10,
                        connect_clicked => PostInput::OpenCreateCommentDialog,
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
        let dialog = CreatePostDialog::builder()
            .transient_for(root)
            .launch_with_broker(DialogType::Comment, &CREATE_COMMENT_DIALOG_BROKER)
            .forward(sender.input_sender(),  |msg| match msg {
                CreatePostDialogOutput::CreateRequest(_name, body) => PostInput::CreateCommentRequest(body)
            });
        let model = PostPage { info: init, image, comments, creator_avatar, community_avatar, create_comment_dialog: dialog, };
        
        let image = model.image.widget();
        let comments = model.comments.widget();
        let creator_avatar = model.creator_avatar.widget();
        let community_avatar = model.community_avatar.widget();
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
                let name = self.info.post_view.creator.name.clone();
                let _ = sender.output(crate::AppMsg::OpenPerson(name));
            }
            PostInput::OpenCommunity => {
                let community_name = self.info.community_view.community.name.clone();
                let _ = sender.output(crate::AppMsg::OpenCommunity(community_name));
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
                CREATE_COMMENT_DIALOG_BROKER.send(DialogMsg::Show)
            }
            PostInput::CreatedComment(comment) => {
                self.comments.guard().push_front(comment);
            }
            PostInput::CreateCommentRequest(body) => {
                let id = self.info.post_view.post.id.0;
                std::thread::spawn(move || {
                    let message = match api::comment::create_comment(id, body, None) {
                        Ok(comment) => Some(PostInput::CreatedComment(comment.comment_view)),
                        Err(err) => { println!("{}", err.to_string()); None }
                    };
                    if message.is_some() { sender.input(message.unwrap()) };
                });
            }
        }
    }
}

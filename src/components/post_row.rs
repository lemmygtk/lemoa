use lemmy_api_common::lemmy_db_views::structs::PostView;
use relm4::prelude::*;
use gtk::prelude::*;
use relm4_components::web_image::WebImage;

use crate::util::get_web_image_url;

#[derive(Debug)]
pub struct PostRow {
    post: PostView,
    author_image: Controller<WebImage>,
    community_image: Controller<WebImage>,
}

#[derive(Debug)]
pub enum PostViewMsg {
    OpenPost,
    OpenCommunity,
    OpenPerson
}

#[relm4::factory(pub)]
impl FactoryComponent for PostRow {
    type Init = PostView;
    type Input = PostViewMsg;
    type Output = crate::AppMsg;
    type CommandOutput = ();
    type Widgets = PostViewWidgets;
    type ParentInput = crate::AppMsg;
    type ParentWidget = gtk::Box;

    view! {
        root = gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 10,
            set_margin_end: 10,
            set_margin_start: 10,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_margin_top: 10,
                set_spacing: 10,
                set_vexpand: false,
                set_hexpand: true,

                if self.post.community.icon.clone().is_some() {
                    gtk::Box {
                        set_hexpand: false,
                        #[local_ref]
                        community_image -> gtk::Box {}
                    }
                } else {
                    gtk::Box {}
                },

                gtk::Button {
                    set_label: &self.post.community.title,
                    connect_clicked => PostViewMsg::OpenCommunity,
                },

                if self.post.creator.avatar.clone().is_some() {
                    gtk::Box {
                        set_hexpand: false,
                        set_margin_start: 10,
                        #[local_ref]
                        author_image -> gtk::Box {}
                    }
                } else {
                    gtk::Box {}
                },

                gtk::Button {
                    set_label: &self.post.creator.name,
                    connect_clicked => PostViewMsg::OpenPerson,
                },

                gtk::Box {
                    set_hexpand: true,
                },

                gtk::Button {
                    set_label: "View",
                    set_margin_end: 10,
                    connect_clicked => PostViewMsg::OpenPost,
                }
            },

            gtk::Label {
                set_halign: gtk::Align::Start,
                set_text: &self.post.post.name,
                add_css_class: "font-bold",
            },

            gtk::Label {
                set_halign: gtk::Align::Start,
                set_text: &format!("{} score, {} comments", self.post.counts.score, self.post.clone().counts.comments),
            },

            gtk::Separator {
                set_margin_top: 10,
            }
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> { Some(output) }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        let author_image= WebImage::builder().launch(get_web_image_url(value.creator.clone().avatar)).detach();
        let community_image= WebImage::builder().launch(get_web_image_url(value.creator.clone().avatar)).detach();

        Self { post: value, author_image, community_image }
    }

    fn init_widgets(
            &mut self,
            _index: &Self::Index,
            root: &Self::Root,
            _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
            sender: FactorySender<Self>,
        ) -> Self::Widgets {
        let author_image = self.author_image.widget();
        let community_image = self.community_image.widget();
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            PostViewMsg::OpenCommunity => {
                sender.output(crate::AppMsg::OpenCommunity(self.post.community.name.clone()))
            }
            PostViewMsg::OpenPerson => {
                sender.output(crate::AppMsg::OpenPerson(self.post.creator.name.clone()))
            }
            PostViewMsg::OpenPost => {
                sender.output(crate::AppMsg::OpenPost(self.post.post.id.clone()))
            }
        }
    }
}
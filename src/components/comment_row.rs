use lemmy_api_common::lemmy_db_views::structs::CommentView;
use relm4::prelude::*;
use gtk::prelude::*;
use relm4_components::web_image::WebImage;

use crate::util::get_web_image_url;
use crate::util::markdown_to_pango_markup;

#[derive(Debug)]
pub struct CommentRow {
    comment: CommentView,
    avatar: Controller<WebImage>
}

#[derive(Debug)]
pub enum CommentRowMsg {
    OpenPerson,
}

#[relm4::factory(pub)]
impl FactoryComponent for CommentRow {
    type Init = CommentView;
    type Input = CommentRowMsg;
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
            set_margin_top: 10,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_vexpand: false,

                if self.comment.creator.avatar.is_some() {
                    gtk::Box {
                        set_hexpand: false,
                        #[local_ref]
                        community_image -> gtk::Box {}
                    }
                } else {
                    gtk::Box {}
                },

                gtk::Button {
                    set_label: &self.comment.creator.name,
                    connect_clicked => CommentRowMsg::OpenPerson,
                },
            },
            
            gtk::Label {
               set_markup: &markdown_to_pango_markup(self.comment.comment.content.clone()),
               set_halign: gtk::Align::Start,
               set_use_markup: true,
            },

            gtk::Label {
                set_label: &format!("{} score", self.comment.counts.score),
                set_halign: gtk::Align::Start,
            },
            
            gtk::Separator {}
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output)
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        let avatar = WebImage::builder().launch(get_web_image_url(value.community.clone().icon)).detach();

        Self { comment: value, avatar }
    }

    fn init_widgets(
            &mut self,
            _index: &Self::Index,
            root: &Self::Root,
            _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
            sender: FactorySender<Self>,
        ) -> Self::Widgets {
        let community_image = self.avatar.widget();
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            CommentRowMsg::OpenPerson => {
                sender.output(crate::AppMsg::OpenPerson(self.comment.creator.name.clone()))
            }
        }
    }
}
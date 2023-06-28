use gtk::prelude::*;
use lemmy_api_common::lemmy_db_views::structs::PrivateMessageView;
use relm4::prelude::FactoryComponent;
use relm4::prelude::*;
use relm4_components::web_image::WebImage;

use crate::util::{self, get_web_image_url, markdown_to_pango_markup};

pub struct PrivateMessageRow {
    message: PrivateMessageView,
    creator_image: Controller<WebImage>,
}

#[derive(Debug)]
pub enum PrivateMessageRowInput {
    OpenPerson,
}

#[relm4::factory(pub)]
impl FactoryComponent for PrivateMessageRow {
    type Init = PrivateMessageView;
    type Input = PrivateMessageRowInput;
    type Output = crate::AppMsg;
    type CommandOutput = ();
    type Widgets = PrivateMessageRowWidgets;
    type ParentInput = crate::AppMsg;
    type ParentWidget = gtk::Box;

    view! {
        root = gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 10,
            set_margin_end: 10,
            set_margin_start: 10,
            set_margin_top: 10,
            set_vexpand: false,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_vexpand: false,
                set_hexpand: true,

                if self.message.creator.avatar.is_some() {
                    gtk::Box {
                        set_hexpand: false,
                        #[local_ref]
                        creator_image -> gtk::Box {}
                    }
                } else {
                    gtk::Box {}
                },

                gtk::Button {
                    set_label: &self.message.creator.name,
                    connect_clicked => PrivateMessageRowInput::OpenPerson,
                },

                gtk::Label {
                    set_margin_start: 10,
                    set_label: &util::format_elapsed_time(self.message.private_message.published)
                }
            },

            gtk::Label {
                #[watch]
               set_markup: &markdown_to_pango_markup(self.message.private_message.content.clone()),
               set_halign: gtk::Align::Start,
               set_use_markup: true,
            },

            gtk::Separator {}
        }
    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        let creator_image = WebImage::builder()
            .launch(get_web_image_url(init.creator.avatar.clone()))
            .detach();
        Self {
            message: init,
            creator_image,
        }
    }
    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let creator_image = self.creator_image.widget();
        let widgets = view_output!();
        widgets
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output)
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            PrivateMessageRowInput::OpenPerson => {
                sender.output(crate::AppMsg::OpenPerson(self.message.creator.id))
            }
        }
    }
}

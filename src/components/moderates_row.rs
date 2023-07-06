use gtk::prelude::*;
use lemmy_api_common::lemmy_db_views_actor::structs::CommunityModeratorView;
use relm4::prelude::*;
use relm4_components::web_image::WebImage;

use crate::util::{get_web_image_url, markdown_to_pango_markup};

#[derive(Debug)]
pub struct ModeratesRow {
    community: CommunityModeratorView,
    community_image: Controller<WebImage>,
}

#[derive(Debug)]
pub enum ModeratesRowMsg {
    OpenCommunity,
}

#[relm4::factory(pub)]
impl FactoryComponent for ModeratesRow {
    type Init = CommunityModeratorView;
    type Input = ModeratesRowMsg;
    type Output = crate::AppMsg;
    type CommandOutput = ();
    type ParentInput = crate::AppMsg;
    type ParentWidget = gtk::Box;

    view! {
        root = gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 10,
            set_margin_end: 10,
            set_margin_start: 10,
            set_vexpand: false,

            add_controller = gtk::GestureClick {
                connect_pressed[sender] => move |_, _, _, _| {
                    sender.input(ModeratesRowMsg::OpenCommunity);
                }
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,

                #[local_ref]
                community_image -> gtk::Box {
                    set_size_request: (35, 35),
                    set_hexpand: false,
                    set_visible: self.community.community.icon.is_some(),
                },

                gtk::Label {
                    set_label: &self.community.community.title,
                },

                gtk::Box {
                    set_hexpand: true,
                },

                gtk::Label {
                    set_label: "NSFW",
                    set_visible: self.community.community.nsfw,
                }
            },

            gtk::Label {
                set_label: &markdown_to_pango_markup(self.community.community.description.clone().unwrap_or("".to_string())),
                set_use_markup: true,
                set_wrap: true,
                set_halign: gtk::Align::Start,
            },

            gtk::Separator {
                set_margin_bottom: 10,
            }
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output)
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        let community_image = WebImage::builder()
            .launch(get_web_image_url(value.community.clone().icon))
            .detach();

        Self {
            community: value,
            community_image,
        }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let community_image = self.community_image.widget();
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            ModeratesRowMsg::OpenCommunity => sender.output(crate::AppMsg::OpenCommunity(
                self.community.community.id.clone(),
            )),
        }
    }
}

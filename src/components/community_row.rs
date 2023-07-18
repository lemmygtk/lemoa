use gtk::prelude::*;
use lemmy_api_common::lemmy_db_views_actor::structs::CommunityView;
use relm4::prelude::*;
use relm4_components::web_image::WebImage;

use crate::util::get_web_image_url;

#[derive(Debug)]
pub struct CommunityRow {
    community: CommunityView,
    community_image: Controller<WebImage>,
}

#[derive(Debug)]
pub enum CommunityRowMsg {
    OpenCommunity,
}

#[relm4::factory(pub)]
impl FactoryComponent for CommunityRow {
    type Init = CommunityView;
    type Input = CommunityRowMsg;
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
                    sender.input(CommunityRowMsg::OpenCommunity);
                }
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,

                #[local_ref]
                community_image -> gtk::Box {
                    set_visible: self.community.community.icon.is_some(),
                    set_hexpand: false,
                },

                gtk::Label {
                    set_label: &self.community.community.title,
                },

                gtk::Box {
                    set_hexpand: true,
                },

                gtk::Label {
                    set_label: &format!("{} subscribers, {} posts", self.community.counts.subscribers, self.community.counts.posts),
                },
            },

            gtk::Separator {}
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
            CommunityRowMsg::OpenCommunity => sender.output(crate::AppMsg::OpenCommunity(
                self.community.community.id,
            )),
        }
    }
}

use lemmy_api_common::lemmy_db_schema::source::instance::Instance;
use relm4::prelude::*;
use gtk::prelude::*;
use relm4_components::web_image::WebImage;

use crate::util::get_web_image_url;

#[derive(Debug)]
pub struct InstanceRow {
    instance: Instance,
    // instance_image: Controller<WebImage>,
}

#[derive(Debug)]
pub enum InstanceRowMsg {
    OpenInstance,
}

#[relm4::factory(pub)]
impl FactoryComponent for InstanceRow {
    type Init = Instance;
    type Input = InstanceRowMsg;
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
            set_vexpand: false,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,

                // if self.instance.instance.icon.is_some() {
                //     gtk::Box {
                //         set_hexpand: false,
                //         #[local_ref]
                //         // instance_image -> gtk::Box {}
                //     }
                // } else {
                //     gtk::Box {}
                // },

                gtk::Label {
                    set_label: &self.instance.domain
                },

                gtk::Box {
                    set_hexpand: true,
                },

                // gtk::Label {
                //     set_label: &format!("{} subscribers, {} posts", self.instance.counts.subscribers, self.instance.counts.posts),
                // },

                gtk::Button {
                    set_label: "View",
                    // connect_clicked => instanceRowMsg::Openinstance,
                },
            },

            gtk::Separator {}
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output)
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        // let instance_image= WebImage::builder().launch(get_web_image_url(value.instance.clone().icon)).detach();

        Self { instance: value }
    }

    fn init_widgets(
            &mut self,
            _index: &Self::Index,
            root: &Self::Root,
            _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
            sender: FactorySender<Self>,
        ) -> Self::Widgets {
        // let instance_image = self.instance_image.widget();
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        // match message {
        //     instanceRowMsg::OpenInstance => {
        //         sender.output(crate::AppMsg::OpenInstance(self.instance.instance.id.clone()))
        //     }
        // }
    }
}
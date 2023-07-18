use gtk::prelude::*;
use lemmy_api_common::lemmy_db_schema::source::instance::Instance;
use relm4::prelude::*;

use super::instances_page::InstancesPageInput;

#[derive(Debug)]
pub struct InstanceRow {
    instance: Instance,
}

#[derive(Debug)]
pub enum InstanceRowMsg {
    OpenInstance,
}

#[relm4::factory(pub)]
impl FactoryComponent for InstanceRow {
    type Init = Instance;
    type Input = InstanceRowMsg;
    type Output = InstancesPageInput;
    type CommandOutput = ();
    type ParentInput = InstancesPageInput;
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
                    sender.input(InstanceRowMsg::OpenInstance);
                    }

            },

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,

                gtk::Label {
                    set_label: &self.instance.domain
                },

                gtk::Box {
                    set_hexpand: true,
                },

            },

            gtk::Separator {}
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output)
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { instance: value }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            InstanceRowMsg::OpenInstance => {
                let instance_address = format!("https://{}", self.instance.domain);
                sender.output(InstancesPageInput::SetInstance(instance_address))
            }
        }
    }
}

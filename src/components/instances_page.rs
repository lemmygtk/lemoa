use gtk::prelude::*;
use lemmy_api_common::lemmy_db_schema::source::instance::Instance;
use relm4::{factory::FactoryVecDeque, prelude::*};

use crate::api;

use super::instance_row::InstanceRow;

pub struct InstancesPage {
    instances: FactoryVecDeque<InstanceRow>,
}

#[derive(Debug)]
pub enum InstancePageInput {
    FetchInstances,
    DoneFetchInstances(Vec<Instance>),
}

#[relm4::component(pub)]
impl SimpleComponent for InstancesPage {
    type Init = ();
    type Input = InstancePageInput;
    type Output = crate::AppMsg;

    view! {
        gtk::Box {
            set_hexpand: true,
            set_vexpand: true,
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 12,
            set_margin_all: 20,
            set_valign: gtk::Align::Center,
            gtk::Label {
                set_text: "Please choose a Lemmy instance",
            },
            gtk::StackSwitcher {
                set_stack: Some(&stack),
            },
            #[name(stack)]
            gtk::Stack {
                add_child = &gtk::ScrolledWindow {
                    #[local_ref]
                    instances -> gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 5,
                        set_vexpand: true,
                    },
                } -> {
                    set_title: "Public",
                },
                add_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    set_margin_top: 100,
                    set_margin_bottom: 100,

                    #[name(instance_url)]
                    gtk::Entry {
                        set_placeholder_text: Some("Instance url"),
                        set_hexpand: true,
                    },
                    gtk::Button {
                        set_label: "Done",
                        connect_clicked[sender, instance_url] => move |_| {
                            let text = instance_url.text().as_str().to_string();
                            instance_url.set_text("");
                            let _ = sender.output(crate::AppMsg::DoneChoosingInstance(text));
                        },
                    },
                } -> {
                    set_title: "Custom",
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let instances = FactoryVecDeque::new(gtk::Box::default(), sender.output_sender());
        let model = Self { instances };
        let instances = model.instances.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            InstancePageInput::FetchInstances => {
                std::thread::spawn(move || {
                    let message = match api::instances::fetch_instances() {
                        Ok(instances) => Some(InstancePageInput::DoneFetchInstances(instances)),
                        Err(_err) => None,
                    };
                    if let Some(message) = message {
                        sender.input(message);
                    };
                });
            }
            InstancePageInput::DoneFetchInstances(instances) => {
                self.instances.guard().clear();
                for instance in instances {
                    self.instances.guard().push_back(instance);
                }
            }
        }
    }
}

use crate::components::loading_indicator::LoadingIndicator;
use gtk::prelude::*;
use lemmy_api_common::site::GetSiteResponse;
use relm4::prelude::*;

use crate::api;

pub struct SiteInfo {
    visible: bool,
    loading: bool,
    site_info: GetSiteResponse,
}

#[derive(Debug)]
pub enum SiteInfoInput {
    Fetch,
    Update(GetSiteResponse),
    Hide,
}

#[relm4::component(pub)]
impl SimpleComponent for SiteInfo {
    type Init = ();
    type Input = SiteInfoInput;
    type Output = crate::AppMsg;

    view! {
        dialog = gtk::Dialog {
            #[watch]
            set_visible: model.visible,
            set_modal: true,

            connect_close_request[sender] => move |_| {
                sender.input(SiteInfoInput::Hide);
                gtk::Inhibit(false)
            },

            gtk::Box {
                match model.loading {
                    true => gtk::Box {
                        #[template]
                        LoadingIndicator,
                    }
                    false => gtk::Box {

                    }
                }
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            visible: false,
            loading: true,
            site_info: api::site::default_site_info(),
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            SiteInfoInput::Fetch => {
                self.loading = true;
                self.visible = true;
                std::thread::spawn(move || match api::site::fetch_site() {
                    Ok(site_info) => sender.input(SiteInfoInput::Update(site_info)),
                    Err(err) => {
                        sender
                            .output_sender()
                            .emit(crate::AppMsg::ShowMessage(err.to_string()));
                        sender.input_sender().emit(SiteInfoInput::Hide);
                    }
                });
            }
            SiteInfoInput::Update(site_info) => {
                self.site_info = site_info;
                self.loading = false;
            }
            SiteInfoInput::Hide => {
                self.visible = false;
            }
        }
    }
}

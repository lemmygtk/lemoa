use crate::{components::loading_indicator::LoadingIndicator, util::get_web_image_msg};
use gtk::prelude::*;
use lemmy_api_common::site::GetSiteResponse;
use relm4::prelude::*;
use relm4_components::web_image::WebImage;

use crate::api;
use crate::util::markdown_to_pango_markup;

pub struct SiteInfo {
    visible: bool,
    loading: bool,
    site_info: GetSiteResponse,
    admin_list: String,
    banner: Controller<WebImage>,
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

            gtk::ScrolledWindow {
                set_size_request: (700, 400),

                match model.loading {
                    true => gtk::Box {
                        #[template]
                        LoadingIndicator,
                    }
                    false => gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 5,
                        set_margin_all: 15,

                        gtk::Label {
                            set_text: &format!("{} v{}", model.site_info.site_view.site.name, model.site_info.version),
                            set_wrap: true,
                        },

                        #[local_ref]
                        banner -> gtk::Box {
                            #[watch]
                            set_visible: model.site_info.site_view.site.banner.is_some(),
                            set_height_request: 300,
                        },

                        gtk::Label {
                            #[watch]
                            set_label: &markdown_to_pango_markup(model.site_info.site_view.site.description.clone().unwrap_or("".to_string())),
                            set_use_markup: true,
                            #[watch]
                            set_visible: model.site_info.site_view.site.description.is_some(),
                            set_wrap: true,
                        },

                        gtk::Label {
                            set_margin_top: 10,
                            #[watch]
                            set_label: &markdown_to_pango_markup(model.site_info.site_view.site.sidebar.clone().unwrap_or("".to_string())),
                            set_use_markup: true,
                            #[watch]
                            set_visible: model.site_info.site_view.site.description.is_some(),
                            set_wrap: true,
                        },

                        gtk::Label {
                            set_margin_top: 10,
                            #[watch]
                            set_text: &format!("{} users, {} posts, {} communities, {} comments",
                                model.site_info.site_view.counts.users, model.site_info.site_view.counts.posts,
                                model.site_info.site_view.counts.communities, model.site_info.site_view.counts.comments),
                            set_wrap: true,
                        },

                        gtk::Label {
                            set_margin_top: 5,
                            #[watch]
                            set_text: &format!("Admins: {}", model.admin_list),
                            set_wrap: true,
                        }
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let banner = WebImage::builder().launch("".to_string()).detach();
        let model = Self {
            visible: false,
            loading: true,
            site_info: api::site::default_site_info(),
            admin_list: String::from(""),
            banner,
        };
        let banner = model.banner.widget();
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
                let banner_url = site_info.site_view.site.banner.clone();
                self.admin_list = site_info
                    .admins
                    .iter()
                    .map(|admin| admin.person.name.clone())
                    .collect::<Vec<String>>()
                    .join(", ");
                self.site_info = site_info;
                self.banner.emit(get_web_image_msg(banner_url));
                self.loading = false;
            }
            SiteInfoInput::Hide => {
                self.visible = false;
            }
        }
    }
}

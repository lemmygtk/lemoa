use crate::config;
use gtk::prelude::GtkWindowExt;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct AboutDialog {}

pub struct Widgets {
    main_window: gtk::Window,
}

impl SimpleComponent for AboutDialog {
    type Input = ();
    type Output = ();
    type Init = gtk::Window;
    type Root = ();
    type Widgets = Widgets;

    fn init_root() -> Self::Root {}

    fn init(
        main_window: Self::Init,
        _root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = Widgets { main_window };

        ComponentParts { model, widgets }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        let dialog = gtk::AboutDialog::builder()
            .icon_name(config::APP_ID)
            .name(config::NAME)
            .authors(["Bnyro"])
            .copyright("© 2023 Lemoa contributors")
            .license_type(gtk::License::Gpl30)
            .version(config::VERSION)
            .modal(true)
            .transient_for(&widgets.main_window)
            .artists(["Bnyro <bnyro@tutanota.com>"])
            .build();
        dialog.present();
    }
}

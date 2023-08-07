use relm4::prelude::*;
use gtk::prelude::*;

#[relm4::widget_template(pub)]
impl WidgetTemplate for LoadingIndicator {
    view! {
        gtk::Box {
            set_hexpand: true,
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 12,
            set_valign: gtk::Align::Center,
            set_halign: gtk::Align::Center,
            gtk::Spinner {
                set_spinning: true,
                set_height_request: 80,
            },
            gtk::Label {
                set_text: "Loading",
            },
        }
    }
}
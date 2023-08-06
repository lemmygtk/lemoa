use lemmy_api_common::lemmy_db_schema::SortType;
use relm4::prelude::*;

pub struct SortDropdown {}

#[derive(Debug)]
pub enum SortDropdownOutput {
    New(SortType),
}

pub struct Widgets {}

impl SimpleComponent for SortDropdown {
    type Init = ();
    type Input = ();
    type Output = SortDropdownOutput;
    type Root = gtk::DropDown;
    type Widgets = Widgets;

    fn init_root() -> Self::Root {
        gtk::DropDown::from_strings(&[
            "Hot",
            "New",
            "Active",
            "Old",
            "Top All",
            "Top Today",
            "Top Week",
            "Top Month",
            "Top Year",
            "Most comments",
            "New comments",
            "Top current hour",
            "Top 6 hours",
        ])
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};
        let dropdown = root.clone();
        dropdown.connect_selected_item_notify(move |dropdown| {
            let sort_type = match dropdown.selected() {
                0 => SortType::Hot,
                1 => SortType::New,
                2 => SortType::Active,
                3 => SortType::Old,
                4 => SortType::TopAll,
                5 => SortType::TopDay,
                6 => SortType::TopWeek,
                7 => SortType::TopMonth,
                8 => SortType::TopYear,
                9 => SortType::MostComments,
                10 => SortType::NewComments,
                11 => SortType::TopHour,
                12 => SortType::TopSixHour,
                _ => SortType::Active,
            };
            sender
                .output_sender()
                .emit(SortDropdownOutput::New(sort_type));
        });
        let widgets = Widgets {};
        ComponentParts { model, widgets }
    }
}

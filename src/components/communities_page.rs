use gtk::prelude::*;
use lemmy_api_common::{
    lemmy_db_schema::ListingType, lemmy_db_views_actor::structs::CommunityView,
};
use relm4::{factory::FactoryVecDeque, prelude::*};

use crate::api;

use super::community_row::CommunityRow;

pub struct CommunitiesPage {
    communities: FactoryVecDeque<CommunityRow>,
    communities_page: i64,
    communities_type: ListingType,
    community_search_buffer: gtk::EntryBuffer,
}

#[derive(Debug)]
pub enum CommunitiesPageInput {
    DoneFetchCommunities(Vec<CommunityView>),
    FetchCommunities(ListingType, bool),
}

#[relm4::component(pub)]
impl SimpleComponent for CommunitiesPage {
    type Init = ();
    type Input = CommunitiesPageInput;
    type Output = crate::AppMsg;

    view! {
        gtk::Box {
            gtk::ScrolledWindow {
                set_vexpand: true,
                set_hexpand: true,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    set_margin_all: 10,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,

                        gtk::ToggleButton {
                            set_label: "All",
                            #[watch]
                            set_active: model.communities_type == ListingType::All,
                            connect_clicked => CommunitiesPageInput::FetchCommunities(ListingType::All, true),
                        },
                        gtk::ToggleButton {
                            set_label: "Local",
                            #[watch]
                            set_active: model.communities_type == ListingType::Local,
                            connect_clicked => CommunitiesPageInput::FetchCommunities(ListingType::Local, true),
                        },
                        gtk::ToggleButton {
                            set_label: "Subscribed",
                            #[watch]
                            set_active: model.communities_type == ListingType::Subscribed,
                            connect_clicked => CommunitiesPageInput::FetchCommunities(ListingType::Subscribed, true),
                        }
                    },

                    gtk::Box {
                        set_spacing: 10,

                        gtk::Entry {
                            set_hexpand: true,
                            set_tooltip_text: Some("Search"),
                            set_buffer: &model.community_search_buffer,
                        },
                        gtk::Button {
                            set_label: "Search",
                            connect_clicked => CommunitiesPageInput::FetchCommunities(model.communities_type, true),
                        }
                    },

                    #[local_ref]
                    communities_box -> gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 5,
                    },

                    gtk::Button {
                        set_label: "More",
                        connect_clicked => CommunitiesPageInput::FetchCommunities(model.communities_type, false),
                        set_margin_top: 10,
                        set_margin_bottom: 10,
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
        let communities = FactoryVecDeque::new(gtk::Box::default(), sender.output_sender());
        let community_search_buffer = gtk::EntryBuffer::builder().build();
        let model = CommunitiesPage {
            communities,
            community_search_buffer,
            communities_page: 1,
            communities_type: ListingType::Local,
        };
        let communities_box = model.communities.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            CommunitiesPageInput::FetchCommunities(listing_type, remove_previous) => {
                let query_text = self.community_search_buffer.text().as_str().to_owned();
                let query = if query_text.is_empty() {
                    None
                } else {
                    Some(query_text)
                };
                let page = if remove_previous {
                    1
                } else {
                    self.communities_page + 1
                };
                self.communities_page = page;
                self.communities_type = listing_type;
                std::thread::spawn(move || {
                    match api::communities::fetch_communities(page, query, Some(listing_type)) {
                        Ok(communities) => {
                            sender.input(CommunitiesPageInput::DoneFetchCommunities(communities));
                        }
                        Err(err) => {
                            sender
                                .output_sender()
                                .emit(crate::AppMsg::ShowMessage(err.to_string()));
                        }
                    };
                });
            }

            CommunitiesPageInput::DoneFetchCommunities(communities) => {
                if self.communities_page == 1 {
                    self.communities.guard().clear();
                }
                for community in communities {
                    self.communities.guard().push_back(community);
                }
            }
        }
    }
}

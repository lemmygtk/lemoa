use lemmy_api_common::person::GetPersonDetailsResponse;
use relm4::{prelude::*, factory::FactoryVecDeque};
use gtk::prelude::*;
use relm4_components::web_image::WebImage;
use std::cell::RefCell;

use crate::util::get_web_image_msg;

use super::post_row::PostRow;

pub struct ProfilePage {
    info: RefCell<GetPersonDetailsResponse>,
    avatar: Controller<WebImage>,
    posts: FactoryVecDeque<PostRow>
}

#[derive(Debug)]
pub enum ProfileInput {
    UpdatePerson(GetPersonDetailsResponse),
}

#[relm4::component(pub)]
impl SimpleComponent for ProfilePage {
    type Init = GetPersonDetailsResponse;
    type Input = ProfileInput;
    type Output = crate::AppMsg;

    view! {
        gtk::ScrolledWindow {
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_vexpand: false,
                set_margin_all: 10,
            
                #[local_ref]
                avatar -> gtk::Box {
                    set_size_request: (100, 100),
                    set_margin_bottom: 20,
                    set_margin_top: 20,
                },
                gtk::Label {
                    #[watch]
                    set_text: &model.info.borrow().person_view.person.name,
                    add_css_class: "font-very-bold",
                },
                gtk::Label {
                    #[watch]
                    set_text: &model.info.borrow().clone().person_view.person.bio.unwrap_or("".to_string()),
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_hexpand: false,
                    set_halign: gtk::Align::Center,

                    gtk::Label {
                        #[watch]
                        set_text: &format!("{} posts, ", model.info.borrow().person_view.counts.post_count),
                    },
                    gtk::Label {
                        #[watch]
                        set_text: &format!("{} comments", model.info.borrow().person_view.counts.comment_count),
                    },
                },

                gtk::Separator {},

                #[local_ref]
                posts -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                }
            }

        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let avatar = WebImage::builder().launch("".to_string()).detach();
        let posts = FactoryVecDeque::new(gtk::Box::default(), sender.output_sender());
        let model = ProfilePage { info: RefCell::new(init), avatar, posts };
        let avatar = model.avatar.widget();
        let posts = model.posts.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            ProfileInput::UpdatePerson(person) => {
                *self.info.borrow_mut() = person.clone();
                self.avatar.emit(get_web_image_msg(person.person_view.person.avatar));
                self.posts.guard().clear();
                for post in person.posts {
                    self.posts.guard().push_back(post);
                }
            }
        }
    }
}

pub mod settings;
pub mod api;
pub mod components;
pub mod util;

use api::{user::default_person, community::default_community, post::default_post};
use components::{post_row::PostRow, community_row::CommunityRow, profile_page::{ProfilePage, self}, community_page::{CommunityPage, self}, post_page::{PostPage, self}};
use gtk::prelude::*;
use lemmy_api_common::{lemmy_db_views_actor::structs::CommunityView, lemmy_db_views::structs::PostView, person::GetPersonDetailsResponse, lemmy_db_schema::newtypes::PostId, post::GetPostResponse, community::GetCommunityResponse};
use relm4::{prelude::*, factory::FactoryVecDeque, set_global_css, actions::{RelmAction, RelmActionGroup}};

static APP_ID: &str = "com.lemmy-gtk.lemoa";

#[derive(Debug, Clone, Copy)]
enum AppState {
    Loading,
    Posts,
    ChooseInstance,
    Communities,
    Community,
    Person,
    Post,
    Login,
    Message
}

struct App {
    state: AppState,
    message: Option<String>,
    latest_action: Option<AppMsg>,
    posts: FactoryVecDeque<PostRow>,
    communities: FactoryVecDeque<CommunityRow>,
    profile_page: Controller<ProfilePage>,
    community_page: Controller<CommunityPage>,
    post_page: Controller<PostPage>
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    ChooseInstance,
    ShowLogin,
    Login(String, String),
    Logout,
    Retry,
    ShowMessage(String),
    DoneChoosingInstance(String),
    StartFetchPosts,
    DoneFetchPosts(Vec<PostView>),
    DoneFetchCommunities(Vec<CommunityView>),
    ViewCommunities(Option<String>),
    OpenCommunity(String),
    DoneFetchCommunity(GetCommunityResponse),
    OpenPerson(String),
    DoneFetchPerson(GetPersonDetailsResponse),
    OpenPost(PostId),
    DoneFetchPost(GetPostResponse)
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        #[root]
        main_window = gtk::ApplicationWindow {
            set_title: Some("Lemoa"),
            set_default_size: (300, 100),

            #[wrap(Some)]
            set_titlebar = &gtk::HeaderBar {
                pack_end =  &gtk::MenuButton {
                    set_icon_name: "view-more",
                    set_menu_model: Some(&menu_model),
                },
                pack_start = &gtk::Button {
                    set_label: "Posts",
                    connect_clicked => AppMsg::StartFetchPosts,
                },
                pack_start = &gtk::Button {
                    set_label: "Communities",
                    connect_clicked => AppMsg::ViewCommunities(None),
                },
            },

            match model.state {
                AppState::Posts => gtk::ScrolledWindow {
                    set_vexpand: true,
                    set_hexpand: true,
                    
                    #[local_ref]
                    posts_box -> gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 5,
                    }
                },
                AppState::Loading => gtk::Box {
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
                },
                AppState::ChooseInstance => gtk::Box {
                    set_hexpand: true,
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 12,
                    set_margin_all: 20,
                    set_valign: gtk::Align::Center,
                    set_halign: gtk::Align::Center,
                    gtk::Label {
                        set_text: "Please enter the URL of a valid lemmy instance",
                    },
                    #[name(instance_url)]
                    gtk::Entry {
                        set_tooltip_text: Some("Instance"),
                    },
                    gtk::Button {
                        set_label: "Done",
                        connect_clicked[sender, instance_url] => move |_| {
                            let text = instance_url.text().as_str().to_string();
                            instance_url.set_text("");
                            sender.input(AppMsg::DoneChoosingInstance(text));
                        },
                    }
                },
                AppState::Login => gtk::Box {
                    set_hexpand: true,
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 12,
                    set_margin_all: 20,
                    set_valign: gtk::Align::Center,
                    set_hexpand: true,

                    gtk::Label {
                        set_text: "Login",
                        add_css_class: "font-bold",
                    },
                    #[name(username)]
                    gtk::Entry {
                        set_placeholder_text: Some("Username or E-Mail"),
                    },
                    #[name(password)]
                    gtk::PasswordEntry {
                        set_placeholder_text: Some("Password"),
                        set_show_peek_icon: true,
                    },
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_halign: gtk::Align::End,

                        gtk::Button {
                            set_label: "Cancel",
                            connect_clicked => AppMsg::StartFetchPosts,
                            set_margin_end: 10,
                        },
                        gtk::Button {
                            set_label: "Login",
                            connect_clicked[sender, username, password] => move |_| {
                                let username_text = username.text().as_str().to_string();
                                username.set_text("");
                                let password_text = password.text().as_str().to_string();
                                password.set_text("");
                                sender.input(AppMsg::Login(username_text, password_text));
                            },
                        },
                    }
                },
                AppState::Communities => gtk::Box {
                    gtk::ScrolledWindow {
                        set_vexpand: true,
                        set_hexpand: true,
                    
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 10,
                            
                            gtk::Box {
                                set_margin_all: 10,

                                #[name(community_search_query)]
                                gtk::Entry {
                                    set_hexpand: true,
                                    set_tooltip_text: Some("Search"),
                                    set_margin_end: 10,
                                },
                                gtk::Button {
                                    set_label: "Search",
                                    connect_clicked[sender, community_search_query] => move |_| {
                                        let text = community_search_query.text().as_str().to_string();
                                        sender.input(AppMsg::ViewCommunities(Some(text)));
                                    },
                                }
                            },

                            #[local_ref]
                            communities_box -> gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 5,
                            }
                        }
                    }
                }
                AppState::Person => {
                    gtk::Box {
                        #[local_ref]
                        profile_page -> gtk::ScrolledWindow {}
                    }
                }
                AppState::Community => {
                    gtk::Box {
                        #[local_ref]
                        community_page -> gtk::ScrolledWindow {}
                    }
                }
                AppState::Post => {
                    gtk::Box {
                        #[local_ref]
                        post_page -> gtk::ScrolledWindow {}
                    }
                }
                AppState::Message => {
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_margin_all: 40,

                        gtk::Label {
                            #[watch]
                            set_text: &model.message.clone().unwrap_or("".to_string()),
                        },
                        gtk::Button {
                            set_label: "Go Home",
                            connect_clicked => AppMsg::Retry,
                        }
                    }
                }
            }
        }
    }

    menu! {
        menu_model: {
            "Choose Instance" => ChangeInstanceAction,
            "Login" => LoginAction,
            "Logout" => LogoutAction
        }
    }

    // Initialize the component.
    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let instance_url = settings::get_prefs().instance_url;
        let state = if instance_url.is_empty() { AppState::ChooseInstance } else { AppState::Loading };

        // initialize all controllers and factories
        let posts = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        let communities = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        let profile_page = ProfilePage::builder().launch(default_person()).forward(sender.input_sender(), |msg| msg);
        let community_page = CommunityPage::builder().launch(default_community()).forward(sender.input_sender(), |msg| msg);
        let post_page = PostPage::builder().launch(default_post()).forward(sender.input_sender(), |msg| msg);
        
        let model = App { state, posts, communities, profile_page, community_page, post_page, message: None, latest_action: None };

        // fetch posts if that's the initial page
        if !instance_url.is_empty() { sender.input(AppMsg::StartFetchPosts) };

        // setup all widgets and different stack pages
        let posts_box = model.posts.widget();
        let communities_box = model.communities.widget();
        let profile_page = model.profile_page.widget();
        let community_page = model.community_page.widget();
        let post_page = model.post_page.widget();
        
        let widgets = view_output!();

        // create the header bar menu and its actions
        let instance_sender = sender.clone();
        let instance_action: RelmAction<ChangeInstanceAction> = RelmAction::new_stateless(move |_| {
            instance_sender.input(AppMsg::ChooseInstance);
        });
        let login_sender = sender.clone();
        let login_action: RelmAction<LoginAction> = RelmAction::new_stateless(move |_| {
            login_sender.input(AppMsg::ShowLogin);
        });
        let logout_action: RelmAction<LogoutAction> = RelmAction::new_stateless(move |_| {
            sender.input(AppMsg::ChooseInstance);
        });

        let mut group = RelmActionGroup::<WindowActionGroup>::new();
        group.add_action(instance_action);
        group.add_action(login_action);
        group.add_action(logout_action);
        group.register_for_widget(&widgets.main_window);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::DoneChoosingInstance(instance_url) => {
                if instance_url.trim().is_empty() { return; }
                let mut preferences = settings::get_prefs();
                preferences.instance_url = instance_url;
                settings::save_prefs(&preferences);
                self.state = AppState::Loading;
                sender.input(AppMsg::StartFetchPosts);
            }
            AppMsg::ChooseInstance => {
                self.state = AppState::ChooseInstance;
            }
            AppMsg::StartFetchPosts => {
                std::thread::spawn(move || {
                    let message = match api::posts::list_posts(1, None) {
                        Ok(posts) => AppMsg::DoneFetchPosts(posts),
                        Err(err) => AppMsg::ShowMessage(err.to_string())
                    };
                    sender.input(message);
                });
            }
            AppMsg::DoneFetchPosts(posts) => {
                self.state = AppState::Posts;
                self.posts.guard().clear();
                for post in posts {
                    self.posts.guard().push_back(post);
                }
            }
            AppMsg::ViewCommunities(query) => {
                self.state = AppState::Communities;
                if (query.is_none() || query.clone().unwrap().trim().is_empty()) && !self.communities.is_empty() { return; }
                std::thread::spawn(move || {
                    let message = match api::communities::fetch_communities(1, query) {
                        Ok(communities) => AppMsg::DoneFetchCommunities(communities),
                        Err(err) => AppMsg::ShowMessage(err.to_string())
                    };
                    sender.input(message);
                });
            }
            AppMsg::DoneFetchCommunities(communities) => {
                self.state = AppState::Communities;
                self.communities.guard().clear();
                for community in communities {
                    self.communities.guard().push_back(community);
                }
            }
            AppMsg::OpenPerson(person_name) => {
                self.state = AppState::Loading;
                std::thread::spawn(move || {
                    let message = match api::user::get_user(person_name, 1) {
                        Ok(person) => AppMsg::DoneFetchPerson(person),
                        Err(err) => AppMsg::ShowMessage(err.to_string())
                    };
                    sender.input(message);
                });
            }
            AppMsg::DoneFetchPerson(person) => {
                self.profile_page.sender().emit(profile_page::ProfileInput::UpdatePerson(person));
                self.state = AppState::Person;
            }
            AppMsg::OpenCommunity(community_name) => {
                self.state = AppState::Loading;
                std::thread::spawn(move || {
                    let message = match api::community::get_community(community_name) {
                        Ok(community) => AppMsg::DoneFetchCommunity(community),
                        Err(err) => AppMsg::ShowMessage(err.to_string())
                    };
                    sender.input(message);
                });
            }
            AppMsg::DoneFetchCommunity(community) => {
                self.community_page.sender().emit(community_page::CommunityInput::UpdateCommunity(community));
                self.state = AppState::Community;
            }
            AppMsg::OpenPost(post_id) => {
                self.state = AppState::Loading;
                std::thread::spawn(move || {
                    let message = match api::post::get_post(post_id) {
                        Ok(post) => AppMsg::DoneFetchPost(post),
                        Err(err) => AppMsg::ShowMessage(err.to_string())
                    };
                    sender.input(message);
                });
            }
            AppMsg::DoneFetchPost(post) => {
                self.post_page.sender().emit(post_page::PostInput::UpdatePost(post));
                self.state = AppState::Post;
            }
            AppMsg::ShowLogin => {
                self.state = AppState::Login;
            }
            AppMsg::Login(username, password) => {
                self.state = AppState::Loading;
                std::thread::spawn(move || {
                    let message = match api::auth::login(username, password) {
                        Ok(login) => {
                            if let Some(token) = login.jwt {
                                util::set_auth_token(Some(token));
                                AppMsg::StartFetchPosts
                            } else {
                                AppMsg::ShowMessage("Wrong credentials!".to_string())
                            }
                        }
                        Err(err) => AppMsg::ShowMessage(err.to_string())
                    };
                    sender.input(message);
                });
            }
            AppMsg::Logout => {
                util::set_auth_token(None);
            }
            AppMsg::ShowMessage(message) => {
                self.message = Some(message);
                self.state = AppState::Message;
            }
            AppMsg::Retry => {
                sender.input(self.latest_action.clone().unwrap_or(AppMsg::StartFetchPosts));
            }
        }
    }
}

relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(ChangeInstanceAction, WindowActionGroup, "instance");
relm4::new_stateless_action!(LoginAction, WindowActionGroup, "login");
relm4::new_stateless_action!(LogoutAction, WindowActionGroup, "logout");

fn main() {
    let app = RelmApp::new(APP_ID);
    set_global_css(include_str!("style.css"));
    app.run::<App>(());
}

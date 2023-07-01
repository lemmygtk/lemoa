pub mod api;
pub mod components;
pub mod config;
pub mod dialogs;
pub mod settings;
pub mod util;

use api::{community::default_community, post::default_post, user::default_person};
use components::{
    communities_page::{CommunitiesPage, CommunitiesPageInput},
    community_page::{self, CommunityPage},
    inbox_page::{InboxInput, InboxPage},
    instances_page::{InstancePageInput, InstancesPage},
    post_page::{self, PostPage},
    post_row::PostRow,
    profile_page::{self, ProfilePage},
};
use dialogs::about::AboutDialog;
use gtk::prelude::*;
use lemmy_api_common::{
    community::GetCommunityResponse,
    lemmy_db_schema::{
        newtypes::{CommunityId, PersonId, PostId},
        ListingType,
    },
    lemmy_db_views::structs::PostView,
    person::GetPersonDetailsResponse,
    post::GetPostResponse,
};
use relm4::{
    actions::{RelmAction, RelmActionGroup},
    factory::FactoryVecDeque,
    prelude::*,
    set_global_css,
};

use crate::components::login_page::LoginPage;

#[derive(Debug, Clone, Copy)]
pub enum AppState {
    Loading,
    Posts,
    ChooseInstance,
    Communities,
    Community,
    Person,
    Post,
    Login,
    Message,
    Inbox,
}

struct App {
    state: AppState,
    message: Option<String>,
    back_queue: Vec<AppMsg>,
    posts: FactoryVecDeque<PostRow>,
    instances_page: Controller<InstancesPage>,
    profile_page: Controller<ProfilePage>,
    community_page: Controller<CommunityPage>,
    communities_page: Controller<CommunitiesPage>,
    post_page: Controller<PostPage>,
    inbox_page: Controller<InboxPage>,
    login_page: Controller<LoginPage>,
    logged_in: bool,
    current_posts_type: Option<ListingType>,
    current_posts_page: i64,
    about_dialog: Controller<AboutDialog>,
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    ChooseInstance,
    ShowLogin,
    LoggedIn,
    Logout,
    ShowMessage(String),
    DoneChoosingInstance(String),
    StartFetchPosts(Option<ListingType>, bool),
    DoneFetchPosts(Vec<PostView>),
    OpenCommunity(CommunityId),
    DoneFetchCommunity(GetCommunityResponse),
    OpenPerson(PersonId),
    DoneFetchPerson(GetPersonDetailsResponse),
    OpenPost(PostId),
    DoneFetchPost(GetPostResponse),
    OpenInbox,
    OpenCommunities,
    PopBackStack,
    UpdateState(AppState),
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
            set_default_size: (1400, 800),

            #[wrap(Some)]
            set_titlebar = &gtk::HeaderBar {
                pack_end =  &gtk::MenuButton {
                    set_icon_name: "view-more",
                    set_menu_model: Some(&menu_model),
                },
                pack_start = &gtk::Button {
                    set_icon_name: "go-previous",
                    connect_clicked => AppMsg::PopBackStack,
                    #[watch]
                    set_visible: model.back_queue.len() > 1,
                },
                pack_start = &gtk::Button {
                    set_label: "Home",
                    connect_clicked => AppMsg::StartFetchPosts(None, true),
                },
                pack_start = &gtk::Button {
                    set_label: "Communities",
                    connect_clicked => AppMsg::OpenCommunities,
                },
                pack_start = &gtk::Button {
                    set_label: "Recommended",
                    connect_clicked => AppMsg::StartFetchPosts(Some(ListingType::Subscribed), true),
                    #[watch]
                    set_visible: model.logged_in,
                },
                pack_start = &gtk::Button {
                    set_label: "Inbox",
                    connect_clicked => AppMsg::OpenInbox,
                    #[watch]
                    set_visible: model.logged_in,
                },
            },

            match model.state {
                AppState::Posts => gtk::ScrolledWindow {
                    set_hexpand: true,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        #[local_ref]
                        posts_box -> gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 5,
                        },

                        gtk::Button {
                            set_label: "More",
                            connect_clicked => AppMsg::StartFetchPosts(model.current_posts_type, false),
                            set_margin_all: 10,
                        }
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
                    #[local_ref]
                    instances_page -> gtk::Box {}
                },
                AppState::Login => gtk::Box {
                    #[local_ref]
                    login_page -> gtk::Box {}
                },
                AppState::Communities => gtk::Box {
                    #[local_ref]
                    communities_page -> gtk::Box {}
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
                            set_label: "Go back",
                            connect_clicked => AppMsg::PopBackStack,
                        }
                    }
                }
                AppState::Inbox => {
                    gtk::ScrolledWindow {
                        #[local_ref]
                        inbox_page -> gtk::Box {}
                    }
                }
            }
        }
    }

    menu! {
        menu_model: {
            "Choose Instance" => ChangeInstanceAction,
            "Profile" => ProfileAction,
            "Login" => LoginAction,
            "Logout" => LogoutAction,
            "About" => AboutAction
        }
    }

    // Initialize the component.
    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let current_account = settings::get_current_account();
        let state = if current_account.instance_url.is_empty() {
            AppState::ChooseInstance
        } else {
            AppState::Loading
        };
        let logged_in = current_account.jwt.is_some();

        // initialize all controllers and factories
        let posts = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        let instances_page = InstancesPage::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| msg);
        let profile_page = ProfilePage::builder()
            .launch(default_person())
            .forward(sender.input_sender(), |msg| msg);
        let community_page = CommunityPage::builder()
            .launch(default_community().community_view)
            .forward(sender.input_sender(), |msg| msg);
        let post_page = PostPage::builder()
            .launch(default_post())
            .forward(sender.input_sender(), |msg| msg);
        let inbox_page = InboxPage::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| msg);
        let communities_page = CommunitiesPage::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| msg);
        let about_dialog = AboutDialog::builder()
            .launch(root.toplevel_window().unwrap())
            .detach();
        let login_page = LoginPage::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| msg);

        let model = App {
            state,
            back_queue: vec![],
            logged_in,
            posts,
            instances_page,
            profile_page,
            community_page,
            post_page,
            inbox_page,
            communities_page,
            login_page,
            message: None,
            current_posts_type: None,
            current_posts_page: 1,
            about_dialog,
        };

        // fetch posts if that's the initial page
        if !current_account.instance_url.is_empty() {
            sender.input(AppMsg::StartFetchPosts(None, true))
        };

        // setup all widgets and different stack pages
        let posts_box = model.posts.widget();
        let instances_page = model.instances_page.widget();
        let profile_page = model.profile_page.widget();
        let community_page = model.community_page.widget();
        let post_page = model.post_page.widget();
        let inbox_page = model.inbox_page.widget();
        let communities_page = model.communities_page.widget();
        let login_page = model.login_page.widget();

        let widgets = view_output!();

        // create the header bar menu and its actions
        let instance_sender = sender.clone();
        let instance_action: RelmAction<ChangeInstanceAction> =
            RelmAction::new_stateless(move |_| {
                instance_sender.input(AppMsg::ChooseInstance);
            });
        let profile_sender = sender.clone();
        let profile_action: RelmAction<ProfileAction> = RelmAction::new_stateless(move |_| {
            let person = settings::get_current_account();
            if !person.name.is_empty() {
                profile_sender.input(AppMsg::OpenPerson(PersonId(person.id)));
            }
        });
        let login_sender = sender.clone();
        let login_action: RelmAction<LoginAction> = RelmAction::new_stateless(move |_| {
            login_sender.input(AppMsg::ShowLogin);
        });
        let logout_action: RelmAction<LogoutAction> = RelmAction::new_stateless(move |_| {
            sender.input(AppMsg::Logout);
        });
        let about_action = {
            let sender = model.about_dialog.sender().clone();
            RelmAction::<AboutAction>::new_stateless(move |_| {
                sender.send(()).unwrap_or_default();
            })
        };

        let mut group = RelmActionGroup::<WindowActionGroup>::new();
        group.add_action(instance_action);
        group.add_action(profile_action);
        group.add_action(login_action);
        group.add_action(logout_action);
        group.add_action(about_action);
        group.register_for_widget(&widgets.main_window);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        // save the back queue
        match msg {
            AppMsg::OpenCommunities
            | AppMsg::DoneFetchCommunity(_)
            | AppMsg::DoneFetchPerson(_)
            | AppMsg::DoneFetchPost(_)
            | AppMsg::DoneFetchPosts(_)
            | AppMsg::ShowMessage(_) => self.back_queue.push(msg.clone()),
            _ => {}
        }

        match msg {
            AppMsg::DoneChoosingInstance(instance_url) => {
                if instance_url.trim().is_empty() {
                    return;
                }
                let url_with_scheme = if instance_url.starts_with("http") {
                    instance_url
                } else {
                    format!("https://{}", instance_url)
                };
                let message = match reqwest::Url::parse(&url_with_scheme) {
                    Ok(url) => {
                        // clear the back queue to not mix up different instances
                        self.back_queue.clear();
                        let mut current_account = settings::get_current_account();
                        let url = url.to_string();
                        // remove the "/" at the end of the url
                        current_account.instance_url = url[0..url.len() - 1].to_string();
                        current_account.jwt = None;
                        settings::update_current_account(current_account);
                        self.state = AppState::Loading;
                        self.logged_in = false;
                        AppMsg::StartFetchPosts(None, true)
                    }
                    Err(err) => AppMsg::ShowMessage(err.to_string()),
                };
                sender.input(message);
            }
            AppMsg::ChooseInstance => {
                self.state = AppState::ChooseInstance;
                self.instances_page
                    .sender()
                    .emit(InstancePageInput::FetchInstances);
            }
            AppMsg::StartFetchPosts(type_, remove_previous) => {
                self.current_posts_type = type_;
                let page = if remove_previous {
                    1
                } else {
                    self.current_posts_page + 1
                };
                // show the loading indicator if it's the first page
                if page == 1 {
                    self.state = AppState::Loading;
                }
                self.current_posts_page = page;
                std::thread::spawn(move || {
                    let message = match api::posts::list_posts(page, None, type_) {
                        Ok(posts) => AppMsg::DoneFetchPosts(posts),
                        Err(err) => AppMsg::ShowMessage(err.to_string()),
                    };
                    sender.input(message);
                });
            }
            AppMsg::DoneFetchPosts(posts) => {
                self.state = AppState::Posts;
                if self.current_posts_page == 1 {
                    self.posts.guard().clear();
                }
                for post in posts {
                    self.posts.guard().push_back(post);
                }
            }
            AppMsg::OpenCommunities => {
                self.state = AppState::Communities;
                self.communities_page
                    .sender()
                    .emit(CommunitiesPageInput::FetchCommunities(
                        ListingType::Local,
                        true,
                    ));
            }
            AppMsg::OpenPerson(person_id) => {
                self.state = AppState::Loading;
                std::thread::spawn(move || {
                    let message = match api::user::get_user(person_id, 1) {
                        Ok(person) => AppMsg::DoneFetchPerson(person),
                        Err(err) => AppMsg::ShowMessage(err.to_string()),
                    };
                    sender.input(message);
                });
            }
            AppMsg::DoneFetchPerson(person) => {
                self.profile_page
                    .sender()
                    .emit(profile_page::ProfileInput::UpdatePerson(person));
                self.state = AppState::Person;
            }
            AppMsg::OpenCommunity(community_id) => {
                self.state = AppState::Loading;
                std::thread::spawn(move || {
                    let message = match api::community::get_community(community_id) {
                        Ok(community) => AppMsg::DoneFetchCommunity(community),
                        Err(err) => AppMsg::ShowMessage(err.to_string()),
                    };
                    sender.input(message);
                });
            }
            AppMsg::DoneFetchCommunity(community) => {
                self.community_page
                    .sender()
                    .emit(community_page::CommunityInput::UpdateCommunity(
                        community.community_view,
                    ));
                self.state = AppState::Community;
            }
            AppMsg::OpenPost(post_id) => {
                self.state = AppState::Loading;
                std::thread::spawn(move || {
                    let message = match api::post::get_post(post_id) {
                        Ok(post) => AppMsg::DoneFetchPost(post),
                        Err(err) => AppMsg::ShowMessage(err.to_string()),
                    };
                    sender.input(message);
                });
            }
            AppMsg::DoneFetchPost(post) => {
                self.post_page
                    .sender()
                    .emit(post_page::PostInput::UpdatePost(post));
                self.state = AppState::Post;
            }
            AppMsg::ShowLogin => {
                self.state = AppState::Login;
            }
            AppMsg::Logout => {
                let mut account = settings::get_current_account();
                account.jwt = None;
                settings::update_current_account(account);
                self.logged_in = false;
            }
            AppMsg::ShowMessage(message) => {
                self.message = Some(message);
                self.state = AppState::Message;
            }
            AppMsg::OpenInbox => {
                self.state = AppState::Inbox;
                self.inbox_page.sender().emit(InboxInput::FetchInbox);
            }
            AppMsg::LoggedIn => {
                self.logged_in = true;
                self.back_queue.clear();
                sender.input(AppMsg::StartFetchPosts(None, true));
            }
            AppMsg::PopBackStack => {
                let action = self.back_queue.get(self.back_queue.len() - 2);
                if let Some(action) = action {
                    sender.input(action.clone());
                }
                for _ in 0..2 {
                    self.back_queue.remove(self.back_queue.len() - 1);
                }
            }
            AppMsg::UpdateState(state) => {
                self.state = state;
            }
        }
    }
}

relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(ChangeInstanceAction, WindowActionGroup, "instance");
relm4::new_stateless_action!(ProfileAction, WindowActionGroup, "profile");
relm4::new_stateless_action!(LoginAction, WindowActionGroup, "login");
relm4::new_stateless_action!(LogoutAction, WindowActionGroup, "logout");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

fn main() {
    let app = RelmApp::new(config::APP_ID);
    set_global_css(include_str!("style.css"));
    app.run::<App>(());
}

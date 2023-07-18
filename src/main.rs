pub mod api;
pub mod components;
pub mod config;
pub mod dialogs;
pub mod settings;
pub mod util;

use api::{community::default_community, post::default_post, user::default_person};
use components::{
    accounts_page::AccountsPage,
    communities_page::{CommunitiesPage, CommunitiesPageInput},
    community_page::{self, CommunityPage},
    inbox_page::{InboxInput, InboxPage},
    instances_page::{InstancesPage, InstancesPageInput},
    post_page::{self, PostPage},
    posts_page::{PostsPage, PostsPageInput},
    profile_page::{ProfileInput, ProfilePage},
};
use dialogs::about::AboutDialog;
use gtk::prelude::*;
use lemmy_api_common::{
    community::GetCommunityResponse,
    lemmy_db_schema::{
        newtypes::{CommunityId, PersonId, PostId},
        ListingType,
    },
    post::GetPostResponse,
};
use relm4::{
    actions::{RelmAction, RelmActionGroup},
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
    Saved,
    AccountsPage,
}

struct App {
    state: AppState,
    message: Option<String>,
    back_queue: Vec<AppMsg>,
    instances_page: Controller<InstancesPage>,
    posts_page: Controller<PostsPage>,
    profile_page: Controller<ProfilePage>,
    community_page: Controller<CommunityPage>,
    communities_page: Controller<CommunitiesPage>,
    post_page: Controller<PostPage>,
    inbox_page: Controller<InboxPage>,
    login_page: Controller<LoginPage>,
    accounts_page: Controller<AccountsPage>,
    saved_page: Controller<ProfilePage>,
    logged_in: bool,
    about_dialog: Controller<AboutDialog>,
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    ChooseInstance,
    LoggedIn,
    Logout,
    ShowMessage(String),
    OpenPosts,
    OpenCommunity(CommunityId),
    DoneFetchCommunity(GetCommunityResponse),
    OpenPerson(PersonId),
    OpenPost(PostId),
    DoneFetchPost(GetPostResponse),
    OpenInbox,
    OpenSaved,
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
                    set_label: "Posts",
                    connect_clicked => AppMsg::OpenPosts,
                },
                pack_start = &gtk::Button {
                    set_label: "Communities",
                    connect_clicked => AppMsg::OpenCommunities,
                },
                pack_start = &gtk::Button {
                    set_label: "Inbox",
                    connect_clicked => AppMsg::OpenInbox,
                    #[watch]
                    set_visible: model.logged_in,
                },
                pack_start = &gtk::Button {
                    set_label: "Saved",
                    connect_clicked => AppMsg::OpenSaved,
                    #[watch]
                    set_visible: model.logged_in,
                },
            },

            match model.state {
                AppState::Posts => gtk::Box {
                    #[local_ref]
                    posts_page -> gtk::ScrolledWindow {}
                }
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
                }
                AppState::ChooseInstance => gtk::Box {
                    #[local_ref]
                    instances_page -> gtk::Box {}
                }
                AppState::Login => gtk::Box {
                    #[local_ref]
                    login_page -> gtk::Box {}
                }
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
                            set_wrap: true,
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
                AppState::Saved => {
                    gtk::Box {
                        #[local_ref]
                        saved_page -> gtk::ScrolledWindow {}
                    }
                }
                AppState::AccountsPage => {
                    gtk::Box {
                        #[local_ref]
                        accounts_page -> gtk::Box {}
                    }
                }
            }
        }
    }

    menu! {
        menu_model: {
            "Change Instance" => ChangeInstanceAction,
            "Accounts" => AccountsAction,
            "Login" => LoginAction,
            "Profile" => ProfileAction,
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

        // initialize all controllers for the various components
        let posts_page = PostsPage::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| msg);
        let instances_page = InstancesPage::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| msg);
        let profile_page = ProfilePage::builder()
            .launch((default_person(), false))
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
        let accounts_page = AccountsPage::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| msg);
        let saved_page = ProfilePage::builder()
            .launch((default_person(), true))
            .forward(sender.input_sender(), |msg| msg);

        let model = App {
            state,
            back_queue: vec![],
            logged_in,
            posts_page,
            instances_page,
            profile_page,
            community_page,
            post_page,
            inbox_page,
            communities_page,
            login_page,
            accounts_page,
            message: None,
            about_dialog,
            saved_page,
        };

        // fetch posts if that's the initial page
        if !current_account.instance_url.is_empty() {
            sender.input(AppMsg::OpenPosts)
        };

        // setup all widgets and different stack pages
        let posts_page = model.posts_page.widget();
        let instances_page = model.instances_page.widget();
        let profile_page = model.profile_page.widget();
        let community_page = model.community_page.widget();
        let post_page = model.post_page.widget();
        let inbox_page = model.inbox_page.widget();
        let communities_page = model.communities_page.widget();
        let login_page = model.login_page.widget();
        let accounts_page = model.accounts_page.widget();
        let saved_page = model.saved_page.widget();

        let widgets = view_output!();

        // create the header bar menu and its actions
        let instance_sender = sender.clone();
        let instance_action: RelmAction<ChangeInstanceAction> =
            RelmAction::new_stateless(move |_| {
                instance_sender.input(AppMsg::ChooseInstance);
            });
        let accounts_sender = sender.clone();
        let accounts_action: RelmAction<AccountsAction> = RelmAction::new_stateless(move |_| {
            accounts_sender.input(AppMsg::UpdateState(AppState::AccountsPage));
        });
        let profile_sender = sender.clone();
        let profile_action: RelmAction<ProfileAction> = RelmAction::new_stateless(move |_| {
            let person = settings::get_current_account();
            if !person.name.is_empty() {
                profile_sender.input(AppMsg::OpenPerson(PersonId(person.id)));
            }
        });
        let login_action: RelmAction<LoginAction> = RelmAction::new_stateless(move |_| {
            sender.input(AppMsg::UpdateState(AppState::Login));
        });
        let about_action = {
            let sender = model.about_dialog.sender().clone();
            RelmAction::<AboutAction>::new_stateless(move |_| {
                sender.send(()).unwrap_or_default();
            })
        };

        let mut group = RelmActionGroup::<WindowActionGroup>::new();
        group.add_action(instance_action);
        group.add_action(accounts_action);
        group.add_action(profile_action);
        group.add_action(login_action);
        group.add_action(about_action);
        group.register_for_widget(&widgets.main_window);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        // save the back queue
        match msg {
            AppMsg::OpenCommunities
            | AppMsg::DoneFetchCommunity(_)
            | AppMsg::OpenPerson(_)
            | AppMsg::DoneFetchPost(_)
            | AppMsg::OpenPosts
            | AppMsg::ShowMessage(_) => self.back_queue.push(msg.clone()),
            _ => {}
        }

        match msg {
            AppMsg::ChooseInstance => {
                self.state = AppState::ChooseInstance;
                self.instances_page
                    .sender()
                    .emit(InstancesPageInput::FetchInstances);
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
                self.profile_page
                    .sender()
                    .emit(ProfileInput::FetchPerson(Some(person_id)));
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
                    .emit(post_page::PostPageInput::UpdatePost(post));
                self.state = AppState::Post;
            }
            AppMsg::Logout => {
                let mut account = settings::get_current_account();
                account.jwt = None;
                account.name = "".to_string();
                account.id = 0;
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
            AppMsg::OpenSaved => {
                let person_id = PersonId(settings::get_current_account().id);
                self.state = AppState::Loading;
                self.saved_page
                    .sender()
                    .emit(ProfileInput::FetchPerson(Some(person_id)));
            }
            AppMsg::LoggedIn => {
                self.logged_in = true;
                self.back_queue.clear();
                sender.input(AppMsg::OpenPosts);
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
            AppMsg::OpenPosts => self
                .posts_page
                .sender()
                .emit(PostsPageInput::FetchPosts(ListingType::Local, true)),
        }
    }
}

relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(ChangeInstanceAction, WindowActionGroup, "instance");
relm4::new_stateless_action!(AccountsAction, WindowActionGroup, "accounts");
relm4::new_stateless_action!(LoginAction, WindowActionGroup, "login");
relm4::new_stateless_action!(ProfileAction, WindowActionGroup, "profile");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

fn main() {
    let app = RelmApp::new(config::APP_ID);
    set_global_css(include_str!("style.css"));
    app.run::<App>(());
}

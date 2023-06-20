use lemmy_api_common::{lemmy_db_schema::{aggregates::structs::{PostAggregates, CommentAggregates}, newtypes::{PostId, CommentId}}};
use relm4::{SimpleComponent, ComponentParts, gtk};
use gtk::prelude::*;

use crate::api;

#[derive(Default, Debug, Clone)]
pub struct VotingStats {
    #[allow(dead_code)]
    upvotes: i64,
    #[allow(dead_code)]
    downvotes: i64,
    score: i64,
    own_vote: Option<i16>,
    #[allow(dead_code)]
    id: i32,
    post_id: Option<i32>,
    comment_id: Option<i32>
}

impl VotingStats {
    pub fn from_post(counts: PostAggregates, my_vote: Option<i16>) -> Self {
        Self {
            upvotes: counts.upvotes,
            downvotes: counts.downvotes,
            own_vote: my_vote,
            post_id: Some(counts.post_id.0),
            id: counts.id,
            score: counts.score,
            comment_id: None,
        }
    }

    pub fn from_comment(counts: CommentAggregates, my_vote: Option<i16>) -> Self {
        Self {
            upvotes: counts.upvotes,
            downvotes: counts.downvotes,
            own_vote: my_vote,
            post_id: None,
            id: counts.id,
            score: counts.score,
            comment_id: Some(counts.comment_id.0),
        }
    }
}

#[derive(Debug)]
pub struct VotingRowModel {
    stats: VotingStats
}

#[derive(Debug)]
pub enum VotingRowInput {
    UpdateStats(VotingStats),
    Vote(i16),
}

#[derive(Debug)]
pub enum VotingRowOutput {

}

#[relm4::component(pub)]
impl SimpleComponent for VotingRowModel {
    type Input = VotingRowInput;
    type Output = VotingRowOutput;
    type Init = VotingStats;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,

            if model.stats.own_vote == Some(1) {
                gtk::Button {
                    set_icon_name: "go-up",
                    connect_clicked => VotingRowInput::Vote(0),
                    add_css_class: "suggested-action",
                }
            } else {
                gtk::Button {
                    set_icon_name: "go-up",
                    connect_clicked => VotingRowInput::Vote(1)
                }
            },
            gtk::Label {
                #[watch]
                set_label: &format!("{}", model.stats.score),
                set_margin_start: 10,
                set_margin_end: 10,
            },
            if model.stats.own_vote == Some(-1) {
                gtk::Button {
                    set_icon_name: "go-down",
                    connect_clicked => VotingRowInput::Vote(0),
                    add_css_class: "suggested-action",
                }
            } else {
                gtk::Button {
                    set_icon_name: "go-down",
                    connect_clicked => VotingRowInput::Vote(-1)
                }
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = VotingRowModel { stats: init };
        let widgets = view_output!();
        ComponentParts { widgets, model }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            VotingRowInput::Vote(score) => {
                let stats = self.stats.clone();
                std::thread::spawn(move || {
                    let info = if stats.post_id.is_some() {
                        let response = api::post::like_post(PostId { 0: stats.post_id.unwrap() }, score);
                        match response {
                            Ok(post) => Some(VotingStats::from_post(post.post_view.counts, post.post_view.my_vote)),
                            Err(err) => { println!("{}", err.to_string()); None }
                        }
                    } else {
                        let response = api::comment::like_comment(CommentId { 0: stats.comment_id.unwrap() }, score);
                        match response {
                            Ok(comment) => Some(VotingStats::from_comment(comment.comment_view.counts, comment.comment_view.my_vote)),
                            Err(err) => { println!("{}", err.to_string()); None }
                        }
                    };
                    if let Some(info) = info { sender.input(VotingRowInput::UpdateStats(info)) };
                });
            }
            VotingRowInput::UpdateStats(stats) => {
                self.stats = stats;
            }
        }
    }
}

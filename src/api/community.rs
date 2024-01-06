use lemmy_api_common::{
    community::{
        BlockCommunity, BlockCommunityResponse, CommunityResponse, FollowCommunity, GetCommunity,
        GetCommunityResponse,
    },
    lemmy_db_schema::newtypes::CommunityId,
};

pub fn get_community(id: CommunityId) -> std::result::Result<GetCommunityResponse, reqwest::Error> {
    let params = GetCommunity {
        id: Some(id),
        ..Default::default()
    };

    super::get("/community", &params)
}

pub fn follow_community(
    community_id: CommunityId,
    follow: bool,
) -> Result<CommunityResponse, reqwest::Error> {
    let params = FollowCommunity {
        community_id,
        follow,
    };
    super::post("/community/follow", &params)
}

pub fn default_community() -> GetCommunityResponse {
    serde_json::from_str(include_str!("../examples/community.json")).unwrap()
}

pub fn block_community(
    community_id: CommunityId,
    block: bool,
) -> std::result::Result<BlockCommunityResponse, reqwest::Error> {
    let params = BlockCommunity {
        community_id,
        block,
    };

    super::post("/community/block", &params)
}

#[derive(Debug, GraphQLInputObject)]
pub struct InputPlayerSession {
    #[graphql(description = "The ID of the Player")]
    pub player_id: i32,

    #[graphql(description = "The amount of money the ")]
    pub buyin: i32,

    #[graphql(description = "The ID of the Session")]
    pub walkout: i32,
}

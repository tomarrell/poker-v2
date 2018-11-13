#[derive(GraphQLInputObject)]
pub struct InputPlayerSession {
    #[graphql(description="The ID of the Player")]
    pub player_id: i32,

    #[graphql(description="The amount of money the ")]
    pub buyin: i32,

    #[graphql(description="The ID of the Session")]
    pub walkout: i32,
}

#[derive(GraphQLInputObject)]
pub struct InputSession {
    #[graphql(description="The ID of the Session")]
    pub id: i32,

    #[graphql(description="The name of the Session")]
    pub name: String,

    #[graphql(description="The ID of the Realm the Session is attached to")]
    pub realm_id: i32,

    #[graphql(description="The time the Session occurred")]
    pub utc_time: String,

    #[graphql(description="A list of PlayerSessions corresponding to each Player participating in the Session")]
    pub player_sessions: Vec<InputPlayerSession>,
}

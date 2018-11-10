# Poker Tracker V2

## Queries
```Go
RealmByName        (args struct{ Name string }) (*RealmResolver, error) {
RealmByID          (args struct{ ID graphql.ID }) (*RealmResolver, error) {
PlayerByID         (args struct{ ID graphql.ID }) (*PlayerResolver, error) {
SessionByID        (args struct{ ID graphql.ID }) (*SessionResolver, error) {
SessionsByRealmID  (args struct{ RealmID graphql.ID }) (*[]*SessionResolver, error) {
```

## Mutations
```
CreateRealm        (args CreateRealm) (*RealmResolver, error) {
CreatePlayer       (args CreatePlayer) (*PlayerResolver, error) {
PutSession         (args CreateSession) (*SessionResolver, error) {
```

## Go Schema
```
type Query {
  realmByName(name: String!): Realm
  realmById(id: ID!): Realm
  sessionById(id: ID!): Session
  sessionsByRealmId(realmId: ID!): [Session]
  playerById(id: ID!): Player
}

type Mutation {
  createRealm(name: String!, title: String): Realm
  createPlayer(name: String!, realmId: ID!): Player
  putSession(id: ID, name: String!, realmId: ID!, time: String!, playerSessions: [CreateSessionPlayerSession]!): Session
}

type Player {
  id: ID!
  name: String!
  realmId: ID!
  playerSessions: [PlayerSession]!
  historicalBalance: Int!
  realBalance: Int!
  totalBuyin: Int!
}

type Realm {
  id: ID!
  name: String!
  title: String
  players: [Player]!
  sessions: [Session]!
}

type Session {
  id: ID!
  realmId: ID!
  name: String
  time: String!
  playerSessions: [PlayerSession]!
}

type PlayerSession {
  player: Player!
  playerId: ID!
  sessionId: ID!
  buyin: Int!
  walkout: Int!
}

input CreateSessionPlayerSession {
  playerId: ID!
  buyin: Int!
  walkout: Int!
}
```

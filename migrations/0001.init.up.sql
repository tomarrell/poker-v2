CREATE TABLE realm(
  id             INTEGER PRIMARY KEY,
  name           TEXT UNIQUE NOT NULL,
  title          TEXT,
  utc_created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE player(
  id             INTEGER PRIMARY KEY,
  name           TEXT NOT NULL,
  realm_id       INTEGER NOT NULL,
  utc_created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,

  FOREIGN KEY (realm_id) REFERENCES realm(id) ON DELETE CASCADE
);

CREATE TABLE session(
  id             INTEGER PRIMARY KEY,
  name           TEXT,
  realm_id       INTEGER NOT NULL,
  utc_time       TEXT NOT NULL,
  utc_created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,

  FOREIGN KEY (realm_id) REFERENCES realm(id) ON DELETE CASCADE
);

CREATE TABLE player_session(
  player_id      INTEGER NOT NULL,
  session_id     INTEGER NOT NULL,
  buyin          INTEGER NOT NULL,
  walkout        INTEGER NOT NULL,
  utc_created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,

  PRIMARY KEY (player_id, session_id),
  FOREIGN KEY (player_id) REFERENCES player(id) ON DELETE CASCADE,
  FOREIGN KEY (session_id) REFERENCES session(id) ON DELETE CASCADE
);

CREATE TABLE transfer(
  id             INTEGER PRIMARY KEY,
  player_id      INTEGER NOT NULL,
  session_id     INTEGER,
  amount         INTEGER NOT NULL,
  reason         TEXT NOT NULL,
  utc_created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,

  FOREIGN KEY (player_id) REFERENCES player(id) ON DELETE CASCADE,
  FOREIGN KEY (session_id) REFERENCES session(id) ON DELETE CASCADE
);

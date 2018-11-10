INSERT INTO realm(name, title) VALUES("Test Realm", "Realm for testing");

INSERT INTO player(name, realm_id) VALUES("Tom", 1);  -- id: 1
INSERT INTO player(name, realm_id) VALUES("Jane", 1); -- id: 2
INSERT INTO player(name, realm_id) VALUES("Sam", 1);  -- id: 3

INSERT INTO session(name, realm_id, utc_time) VALUES("Test Session", 1, "2018-11-10T00:00:00.000Z"); -- id: 1

INSERT INTO player_session(player_id, session_id, buyin, walkout) VALUES(1, 1, 100, 200);
INSERT INTO player_session(player_id, session_id, buyin, walkout) VALUES(2, 1, 100, 0);

INSERT INTO transfer(player_id, session_id, amount, reason) VALUES(1, 1, 100, "session_participation");
INSERT INTO transfer(player_id, session_id, amount, reason) VALUES(2, 1, -100, "session_participation");


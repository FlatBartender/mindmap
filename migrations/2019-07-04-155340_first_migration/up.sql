-- Your SQL goes here

CREATE TYPE permission AS ENUM (
	'Read',
	'Write',
	'Moderate',
	'Admin',
	'Owner'
);

CREATE TYPE edgestyle AS ENUM (
	'Dotted',
	'Full',
	'Dashed'
);

CREATE TABLE mindmaps (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL,
	default_permission permission NOT NULL
);

CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	username TEXT NOT NULL UNIQUE,
	password TEXT NOT NULL,
	created TIMESTAMP
);

CREATE TABLE permissions (
	map integer REFERENCES mindmaps,
	user_id integer REFERENCES users,
	type permission NOT NULL,
	PRIMARY KEY (map, user_id)
);

CREATE TABLE nodes (
	map integer REFERENCES mindmaps,
	id SERIAL,
	text TEXT NOT NULL,
	color TEXT NOT NULL,
	tags TEXT NOT NULL,
	PRIMARY KEY (map, id)
);

CREATE TABLE edges (
	map integer REFERENCES mindmaps,
	id SERIAL,
	node_from integer,
	node_to integer,
	brief TEXT NOT NULL,
	description TEXT NOT NULL,
	style edgestyle NOT NULL,
	PRIMARY KEY (map, id),
	FOREIGN KEY (map, node_from) REFERENCES nodes (map, id),
	FOREIGN KEY (map, node_to) REFERENCES nodes (map, id)
);

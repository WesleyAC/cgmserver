CREATE TABLE entries(
	type_       TEXT,
	date_string TEXT,
	date_       INTEGER PRIMARY KEY NOT NULL,
	sgv         REAL,
	direction   TEXT,
	noise       REAL,
	filtered    REAL,
	unfiltered  REAL,
	rssi        REAL
);

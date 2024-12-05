CREATE SCHEMA cti;

DROP TABLE IF EXISTS cti.cves;
CREATE TABLE IF NOT EXISTS cti.cves (
    id text PRIMARY KEY,
    state text NOT NULL,
	date_published text NOT null,
	description text NOT NULL,

    _created_at timestamp with time zone DEFAULT now()
);
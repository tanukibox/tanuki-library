CREATE SCHEMA cti;

DROP TABLE IF EXISTS cti.cves;
CREATE TABLE IF NOT EXISTS cti.cves (
    id text PRIMARY KEY,
    state text NOT NULL,
	description text NOT NULL,
    assigner_id text NOT NULL,
    assigner_name text NOT NULL,
    date_published text NOT null,
    date_updated text NOT null,

    _created_at timestamp with time zone DEFAULT now()
);
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

DROP TABLE IF EXISTS cti.breaches;
CREATE TABLE IF NOT EXISTS cti.breaches (
    id text NOT NULL,
    vendor text PRIMARY KEY,
    product text PRIMARY KEY,
    product_version text PRIMARY KEY,
    product_type text NOT NULL,
    
    cve_id text PRIMARY KEY,
    cve_state text NOT NULL,
	cve_description text NOT NULL,
    cve_assigner_id text NOT NULL,
    cve_assigner_name text NOT NULL,
    cve_date_published text NOT null,
    cve_date_updated text NOT null,

    _created_at timestamp with time zone DEFAULT now()
);
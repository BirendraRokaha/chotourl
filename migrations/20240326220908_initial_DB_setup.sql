-- Add migration script here
CREATE TABLE IF NOT EXISTS chotourls (
  url_id TEXT PRIMARY KEY,
  org_url varchar NOT NULL,
  short_url TEXT NOT NULL,
  cust_phrase TEXT,
  inserted_at TIMESTAMPTZ NOT NULL default now(),
  updated_at TIMESTAMPTZ NOT NULL default now(),
  visits SMALLINT NOT NULL,
  url_code INT NOT NULL
);

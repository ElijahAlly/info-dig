-- Your SQL goes here
-- up.sql
ALTER TABLE statements ADD COLUMN slug VARCHAR NOT NULL;

DROP INDEX IF EXISTS index_statements_on_context;

CREATE INDEX index_statements_on_slug ON statements(slug);

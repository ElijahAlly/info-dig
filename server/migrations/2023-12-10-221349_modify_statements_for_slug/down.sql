-- This file should undo anything in `up.sql`
-- down.sql
ALTER TABLE statements DROP COLUMN slug;

CREATE INDEX index_statements_on_context ON statements(context);

DROP INDEX IF EXISTS index_statements_on_slug;

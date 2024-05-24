SELECT c.relnamespace::regnamespace AS scope_name,
       c.relname AS table_name
FROM pg_catalog.pg_class c
    LEFT OUTER JOIN pg_catalog.pg_index i ON c.oid = i.indrelid AND i.indisprimary
WHERE i.indkey IS NULL;

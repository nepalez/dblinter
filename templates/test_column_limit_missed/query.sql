SELECT t.relnamespace::regnamespace AS scope_name,
       a.attrelid::regclass AS table_name,
       a.attname AS column_name,
       {{ limit }} AS limit
FROM pg_attribute a
    INNER JOIN pg_class t
        ON a.attrelid = t.oid
    LEFT OUTER JOIN pg_constraint c
        ON c.conrelid = a.attrelid
        AND c.conkey = a.attnum
        AND c.contype = 'p'
WHERE c.contype IS NULL;

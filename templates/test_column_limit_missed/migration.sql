ALTER TABLE {{ scope_name }}.{{ table_name }}
    ADD CONSTRAINT {{ column_name }}_limit
    CHECK (lenght({{ column_name }}) <= {{ limit }});
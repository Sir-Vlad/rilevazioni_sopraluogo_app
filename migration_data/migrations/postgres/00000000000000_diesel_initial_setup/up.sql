-- this file was automatically created by diesel to setup helper functions
-- and other internal bookkeeping. this file is safe to edit, any future
-- changes will be added to existing projects as new migrations.

create or replace function diesel_manage_updated_at(_tbl regclass) returns void as
$$
begin
    execute format('create trigger set_updated_at before update on %s
                    for each row execute procedure diesel_set_updated_at()', _tbl);
end;
$$ language plpgsql;

create or replace function diesel_set_updated_at() returns trigger as
$$
begin
    if (
        new is distinct from old and
        new.updated_at is not distinct from old.updated_at
        ) then
        new.updated_at := current_timestamp;
    end if;
    return new;
end;
$$ language plpgsql;

create or replace function validate_not_empty(text_value text, field_text text)
    returns boolean as
$$
begin
    if text_value is null or length(trim(text_value)) = 0 then
        raise exception 'Field % cannot be empty or contain only whitespace', field_text;
    end if;
    return true;
end
$$ language plpgsql;
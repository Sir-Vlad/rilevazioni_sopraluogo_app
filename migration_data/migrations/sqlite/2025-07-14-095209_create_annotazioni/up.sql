create table if not exists annotazione_edificio
(
    id          integer primary key autoincrement,
    edificio_id text not null,
    content     text not null check ( content is not null and trim(content) <> ''),
    data        text not null default current_timestamp,
    foreign key (edificio_id) references edificio (chiave)
);

create table if not exists annotazione_stanza
(
    id        integer primary key autoincrement,
    stanza_id integer not null,
    content   text    not null check ( content is not null and trim(content) <> ''),
    data      text    not null default current_timestamp,
    foreign key (stanza_id) references stanza (id)
);

create table if not exists annotazione_infisso
(
    id          integer primary key autoincrement,
    infisso_id  text not null,
    edificio_id text not null,
    content     text not null check ( content is not null and trim(content) <> ''),
    data        text not null default current_timestamp,
    foreign key (infisso_id, edificio_id) references infisso (id, edificio_id)
);
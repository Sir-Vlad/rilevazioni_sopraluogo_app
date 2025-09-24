create table if not exists annotazione_edificio
(
    id          serial primary key,
    edificio_id varchar(10) not null,
    content     text        not null check ( validate_not_empty(content, 'content') ),
    data        timestamp   not null default current_timestamp,
    foreign key (edificio_id) references edificio (chiave) on delete cascade on update cascade
);

create table if not exists annotazione_stanza
(
    id        serial primary key,
    stanza_id integer   not null,
    content   text      not null check ( validate_not_empty(content, 'content') ),
    data      timestamp not null default current_timestamp,
    foreign key (stanza_id) references stanza (id) on delete cascade on update cascade
);

create table if not exists annotazione_infisso
(
    id          serial primary key,
    infisso_id  varchar(4)  not null,
    edificio_id varchar(10) not null,
    content     text        not null check ( validate_not_empty(content, 'content') ),
    data        timestamp   not null default current_timestamp,
    foreign key (infisso_id, edificio_id) references infisso (id, edificio_id) on delete cascade on update cascade
);
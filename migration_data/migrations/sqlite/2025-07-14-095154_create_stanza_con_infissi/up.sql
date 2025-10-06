create table if not exists stanza_con_infissi
(
    infisso_id  text    not null,
    edificio_id text    not null,
    stanza_id   integer not null,
    num_infisso integer not null check ( num_infisso >= 0 ),
    primary key (infisso_id, edificio_id, stanza_id),
    foreign key (infisso_id, edificio_id) references infisso (id, edificio_id),
    foreign key (stanza_id) references stanza (id)
);

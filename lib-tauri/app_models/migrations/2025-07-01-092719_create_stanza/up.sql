create table if not exists stanza
(
    id               serial primary key,
    edificio_id      varchar(10) not null references edificio (chiave) on delete CASCADE on update CASCADE,
    piano            char(2)     not null,
    id_spazio        text        not null,
    cod_stanza       varchar(10) not null,
    destinazione_uso varchar(20) not null,
    altezza          smallint check ( altezza >= 0 ),
    spessore_muro    smallint check ( spessore_muro >= 0 ),
    riscaldamento    varchar(20),
    raffrescamento   varchar(20),
    illuminazione    varchar(20),
    unique (edificio_id, id_spazio, cod_stanza, destinazione_uso),
    foreign key (riscaldamento) references climatizzazione (nome) on delete set null on update cascade,
    foreign key (raffrescamento) references climatizzazione (nome) on delete set null on update cascade,
    foreign key (illuminazione) references illuminazione (lampadina) on delete set null on update cascade
);

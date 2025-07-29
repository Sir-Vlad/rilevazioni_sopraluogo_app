create table if not exists stanza
(
    id               integer primary key autoincrement,
    edificio_id      text not null references edificio (chiave),
    piano            text not null,
    id_spazio        text not null,
    cod_stanza       text not null check ( length(cod_stanza) <= 10 ),
    destinazione_uso text not null check ( length(destinazione_uso) <= 15 ),
    altezza          integer check ( altezza >= 0 ),
    spessore_muro    integer check ( spessore_muro >= 0 ),
    riscaldamento    text check ( length(riscaldamento) <= 20 ),
    raffrescamento   text check ( length(raffrescamento) <= 20 ),
    illuminazione    text check ( length(illuminazione) <= 20 ),
    unique (edificio_id, id_spazio, cod_stanza, destinazione_uso),
    foreign key (riscaldamento) references climatizzazione (nome),
    foreign key (raffrescamento) references climatizzazione (nome),
    foreign key (illuminazione) references illuminazione (lampadina)
);

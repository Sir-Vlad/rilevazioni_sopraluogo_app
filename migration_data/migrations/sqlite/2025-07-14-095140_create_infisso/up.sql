create table if not exists infisso
(
    id          text    not null check ( length(id) <= 4 ),
    edificio_id text    not null check ( length(edificio_id) <= 7 ),
    tipo        text    not null check ( length(tipo) <= 20 ),
    altezza     integer not null,
    larghezza   integer not null,
    materiale   text    not null check ( length(tipo) <= 20 ),
    vetro       text    not null check ( length(tipo) <= 20 ),
    mq          real generated always as ( (altezza * larghezza) / 10000.0 ) virtual,
    primary key (id, edificio_id),
    foreign key (edificio_id) references edificio (chiave),
    foreign key (tipo) references tipo_infisso (nome),
    foreign key (materiale) references materiale_infisso (materiale),
    foreign key (vetro) references vetro_infisso (vetro)
);

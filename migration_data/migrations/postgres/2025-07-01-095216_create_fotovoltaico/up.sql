create table if not exists fotovoltaico
(
    id           serial primary key,
    edificio_id  char(7)     not null,
    potenza      real        not null check ( potenza > 0 ),
    proprietario varchar(50) not null check ( validate_not_empty(proprietario, 'proprietario') ),
    foreign key (edificio_id) references edificio (chiave)
);

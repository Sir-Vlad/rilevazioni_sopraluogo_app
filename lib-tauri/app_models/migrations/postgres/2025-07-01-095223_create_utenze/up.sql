create type tipo_utenza as enum ('acqua', 'elettricità', 'riscaldamento');

create table if not exists utenze
(
    id                  serial primary key,
    edificio_id         char(7)     not null,
    tipo                tipo_utenza not null,
    cod_contatore       varchar(20) not null check ( validate_not_empty(cod_contatore, 'cod_contatore') ),
    indirizzo_contatore varchar(50),
    foreign key (edificio_id) references edificio (chiave)
)

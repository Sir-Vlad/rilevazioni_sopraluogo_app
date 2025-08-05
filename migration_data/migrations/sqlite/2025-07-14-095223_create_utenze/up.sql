create table if not exists utenze
(
    id                  integer primary key autoincrement,
    edificio_id         text not null,
    tipo                text not null check ( tipo in ('acqua', 'elettricità', 'riscaldamento') ),
    cod_contatore       text not null check ( cod_contatore is not null and trim(cod_contatore) <> '' and
                                              length(cod_contatore) <= 20 ),
    indirizzo_contatore text check ( length(indirizzo_contatore) <= 50 ),
    foreign key (edificio_id) references edificio (chiave)
)

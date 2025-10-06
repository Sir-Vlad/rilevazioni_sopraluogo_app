create table if not exists fotovoltaico
(
    id           integer primary key autoincrement,
    edificio_id  text not null,
    potenza      real not null check ( potenza > 0 ),
    proprietario text not null check ( proprietario is not null and trim(proprietario) <> '' and
                                       length(proprietario) <= 50 ),
    foreign key (edificio_id) references edificio (chiave)
);

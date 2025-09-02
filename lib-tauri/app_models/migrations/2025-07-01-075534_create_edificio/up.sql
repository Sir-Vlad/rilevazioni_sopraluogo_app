create table if not exists edificio
(
    chiave                varchar(10) primary key,
    fascicolo             integer not null,
    indirizzo             text    not null,
    anno_costruzione      integer check ( anno_costruzione >= 1900 and anno_costruzione <= 2100 )           default null,
    anno_riqualificazione integer check ( anno_riqualificazione >= 1900 and anno_riqualificazione <= 2100 ) default null,
    note_riqualificazione text                                                                              default null,
    isolamento_tetto      boolean not null                                                                  default false,
    cappotto              boolean not null                                                                  default false
);

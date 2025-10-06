create table if not exists climatizzazione
(
    nome           text primary key check ( nome is not null and trim(nome) <> '' ),
    eff_energetica integer not null
);

insert into climatizzazione
values ('No climatizzazione', 0),
       ('Radiatori', 1),
       ('Ventilconvettori', 2),
       ('Split', 3),
       ('A pavimento', 4),
       ('Pannelli radianti', 5),
       ('Bocchette ad aria', 6);

create table if not exists illuminazione
(
    lampadina      text primary key check ( lampadina is not null and trim(lampadina) <> '' ),
    eff_energetica integer not null
);

insert into illuminazione
values ('No illuminata', 0),
       ('Alogene', 1),
       ('Neon', 2),
       ('Led', 3),
       ('Fluorescenza', 4);

create table if not exists materiale_infisso
(
    materiale      text primary key check ( materiale is not null and trim(materiale) <> '' ),
    eff_energetica integer not null
);

insert into materiale_infisso
values ('Legno', 1),
       ('Ferro', 2),
       ('Alluminio', 3),
       ('PVC', 4);

create table if not exists vetro_infisso
(
    vetro          text primary key check ( vetro is not null and trim(vetro) <> '' ),
    eff_energetica integer not null
);

insert into vetro_infisso
values ('Singolo', 1),
       ('Doppio', 2),
       ('Camera', 3),
       ('Triplo', 4);

create table if not exists tipo_infisso
(
    nome text primary key check ( nome is not null and trim(nome) <> '' )
);

insert into tipo_infisso
values ('Finestra'),
       ('Porta'),
       ('Vetrata'),
       ('Porta-finestra'),
       ('Lucernario');
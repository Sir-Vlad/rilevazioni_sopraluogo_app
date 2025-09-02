create table if not exists climatizzazione
(
    nome           varchar(20) primary key check ( validate_not_empty(nome, 'name') ),
    eff_energetica smallint not null
);

insert into climatizzazione
values ('No climatizzata', 0),
       ('Radiatori', 1),
       ('Ventilconvettori', 2),
       ('Split', 3),
       ('A pavimento', 4),
       ('Pannelli radianti', 5),
       ('Bocchette ad aria', 6);

create table if not exists illuminazione
(
    lampadina      varchar(20) primary key check ( validate_not_empty(lampadina, 'lampadina') ),
    eff_energetica smallint not null
);

insert into illuminazione
values ('No illuminata', 0),
       ('Alogene', 1),
       ('Neon', 2),
       ('Led', 3),
       ('Fluorescenza', 4);

create table if not exists materiale_infisso
(
    materiale      varchar(20) primary key check ( validate_not_empty(materiale, 'materiale') ),
    eff_energetica smallint not null
);

insert into materiale_infisso
values ('Legno', 1),
       ('Ferro', 2),
       ('Alluminio', 3),
       ('PVC', 4);

create table if not exists vetro_infisso
(
    vetro          varchar(20) primary key check ( validate_not_empty(vetro, 'vetro') ),
    eff_energetica smallint not null
);

insert into vetro_infisso
values ('Singolo', 1),
       ('Doppio', 2),
       ('Camera', 3),
       ('Triplo', 4),
       ('Plexiglas', 5);

create table if not exists tipo_infisso
(
    nome varchar(20) primary key check ( validate_not_empty(nome, 'nome') )
);

insert into tipo_infisso
values ('Finestra'),
       ('Porta'),
       ('Vetrata'),
       ('Porta-finestra'),
       ('Lucernario');
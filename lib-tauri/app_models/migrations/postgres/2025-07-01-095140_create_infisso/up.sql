create table if not exists infisso
(
    id          char(4)     not null,
    edificio_id varchar(10)    not null,
    tipo        varchar(20) not null,
    altezza     smallint    not null check ( altezza >= 0 ),
    larghezza   smallint    not null check ( larghezza >= 0 ),
    materiale   varchar(20) not null,
    vetro       varchar(20) not null,
    mq          real        not null,
    primary key (id, edificio_id),
    foreign key (edificio_id) references edificio (chiave),
    foreign key (tipo) references tipo_infisso (nome),
    foreign key (materiale) references materiale_infisso (materiale),
    foreign key (vetro) references vetro_infisso (vetro)
);

CREATE OR REPLACE FUNCTION calculate_mq() RETURNS trigger AS
$$
BEGIN
    NEW.mq = (NEW.altezza::real * NEW.larghezza::real) / 10000.0;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Crea il trigger
CREATE TRIGGER infisso_calculate_mq
    BEFORE INSERT OR UPDATE
    ON infisso
    FOR EACH ROW
EXECUTE FUNCTION calculate_mq();

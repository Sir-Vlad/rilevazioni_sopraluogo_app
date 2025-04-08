PRAGMA foreign_keys = ON;

INSERT OR IGNORE INTO EDIFICIO(CHIAVE, FASCICOLO, INDIRIZZO)
VALUES ('8406-18', '00004533', 'VIA POLICARPO SCARABELLO');


INSERT OR IGNORE INTO STANZA(CHIAVE, PIANO, ID_SPAZIO, STANZA, DESTINAZIONE_USO)
VALUES ('8406-18', '-1', '05687', '-100', 'Atrio'),
       ('8406-18', '-1', '05688', '-101', 'Ufficio'),
       ('8406-18', '-1', '05689', '-102', 'Magazzino'),
       ('8406-18', '-1', '05690', '-103', 'Vano tecnico'),
       ('8406-18', '-1', '05691', '-104', 'Sala mensa');


INSERT OR IGNORE INTO MATERIALE_INFISSO(MATERIALE, EFFICIENZA_ENERGETICA)
VALUES ('Legno', 1),
       ('Alluminio', 2),
       ('PVC', 3);

INSERT OR IGNORE INTO VETRO_INFISSO(VETRO, EFFICIENZA_ENERGETICA)
VALUES ('Singolo', 1),
       ('Doppio', 2),
       ('Camera', 3);

INSERT OR IGNORE INTO INFISSO(ID, ALTEZZA, LARGHEZZA, MATERIALE, VETRO)
VALUES ('A', 52, 85, 'Alluminio', 'Doppio'),
       ('B', 80, 150, 'Legno', 'Singolo'),
       ('C', 250, 63, 'Legno', 'Doppio'),
       ('D', 12, 12, 'PVC', 'Camera');

INSERT OR IGNORE INTO STANZE_CON_INFISSI(ID_INFISSO, ID_STANZA, RIPETIZIONE)
VALUES ('A', '1', 5),
       ('B', 1, 1),
       ('A', 2, 1),
       ('C', 2, 2);

INSERT OR IGNORE INTO ILLUMINAZIONE(LAMPADINA, EFFICIENZA_ENERGETICA)
VALUES ('Alogene', 1),
       ('Neon', 2),
       ('Led', 3),
       ('Fluorescenza', 4);
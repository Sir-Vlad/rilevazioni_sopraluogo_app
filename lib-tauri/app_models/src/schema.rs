// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tipo_utenza"))]
    pub struct TipoUtenza;
}

diesel::table! {
    annotazione_edificio (id) {
        id -> Int4,
        #[max_length = 7]
        edificio_id -> Varchar,
        content -> Text,
        data -> Timestamp,
    }
}

diesel::table! {
    annotazione_infisso (id) {
        id -> Int4,
        #[max_length = 4]
        infisso_id -> Varchar,
        #[max_length = 7]
        edificio_id -> Varchar,
        content -> Text,
        data -> Timestamp,
    }
}

diesel::table! {
    annotazione_stanza (id) {
        id -> Int4,
        stanza_id -> Int4,
        content -> Text,
        data -> Timestamp,
    }
}

diesel::table! {
    climatizzazione (nome) {
        #[max_length = 20]
        nome -> Varchar,
        eff_energetica -> Int2,
    }
}

diesel::table! {
    edificio (chiave) {
        #[max_length = 7]
        chiave -> Bpchar,
        fascicolo -> Int4,
        indirizzo -> Text,
        anno_costruzione -> Nullable<Int4>,
        anno_riqualificazione -> Nullable<Int4>,
        note_riqualificazione -> Nullable<Text>,
        isolamento_tetto -> Bool,
        cappotto -> Bool,
    }
}

diesel::table! {
    fotovoltaico (id) {
        id -> Int4,
        #[max_length = 7]
        edificio_id -> Bpchar,
        potenza -> Float4,
        #[max_length = 50]
        proprietario -> Varchar,
    }
}

diesel::table! {
    illuminazione (lampadina) {
        #[max_length = 20]
        lampadina -> Varchar,
        eff_energetica -> Int2,
    }
}

diesel::table! {
    infisso (id, edificio_id) {
        #[max_length = 4]
        id -> Bpchar,
        #[max_length = 7]
        edificio_id -> Bpchar,
        #[max_length = 20]
        tipo -> Varchar,
        altezza -> Int2,
        larghezza -> Int2,
        #[max_length = 20]
        materiale -> Varchar,
        #[max_length = 20]
        vetro -> Varchar,
        mq -> Float4,
    }
}

diesel::table! {
    materiale_infisso (materiale) {
        #[max_length = 20]
        materiale -> Varchar,
        eff_energetica -> Int2,
    }
}

diesel::table! {
    stanza (id) {
        id -> Int4,
        #[max_length = 7]
        edificio_id -> Bpchar,
        #[max_length = 1]
        piano -> Bpchar,
        id_spazio -> Text,
        #[max_length = 10]
        cod_stanza -> Varchar,
        #[max_length = 15]
        destinazione_uso -> Varchar,
        altezza -> Nullable<Int2>,
        spessore_muro -> Nullable<Int2>,
        #[max_length = 20]
        riscaldamento -> Nullable<Varchar>,
        #[max_length = 20]
        raffrescamento -> Nullable<Varchar>,
        #[max_length = 20]
        illuminazione -> Nullable<Varchar>,
    }
}

diesel::table! {
    stanza_con_infissi (infisso_id, edificio_id, stanza_id) {
        #[max_length = 4]
        infisso_id -> Bpchar,
        #[max_length = 7]
        edificio_id -> Bpchar,
        stanza_id -> Int4,
        num_infisso -> Int4,
    }
}

diesel::table! {
    tipo_infisso (nome) {
        #[max_length = 20]
        nome -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TipoUtenza;

    utenze (id) {
        id -> Int4,
        #[max_length = 7]
        edificio_id -> Bpchar,
        tipo -> TipoUtenza,
        #[max_length = 20]
        cod_contatore -> Varchar,
        #[max_length = 50]
        indirizzo_contatore -> Nullable<Varchar>,
    }
}

diesel::table! {
    vetro_infisso (vetro) {
        #[max_length = 20]
        vetro -> Varchar,
        eff_energetica -> Int2,
    }
}

diesel::joinable!(annotazione_edificio -> edificio (edificio_id));
diesel::joinable!(annotazione_stanza -> stanza (stanza_id));
diesel::joinable!(fotovoltaico -> edificio (edificio_id));
diesel::joinable!(infisso -> edificio (edificio_id));
diesel::joinable!(infisso -> materiale_infisso (materiale));
diesel::joinable!(infisso -> tipo_infisso (tipo));
diesel::joinable!(infisso -> vetro_infisso (vetro));
diesel::joinable!(stanza -> edificio (edificio_id));
diesel::joinable!(stanza -> illuminazione (illuminazione));
diesel::joinable!(stanza_con_infissi -> stanza (stanza_id));
diesel::joinable!(utenze -> edificio (edificio_id));

diesel::allow_tables_to_appear_in_same_query!(
    annotazione_edificio,
    annotazione_infisso,
    annotazione_stanza,
    climatizzazione,
    edificio,
    fotovoltaico,
    illuminazione,
    infisso,
    materiale_infisso,
    stanza,
    stanza_con_infissi,
    tipo_infisso,
    utenze,
    vetro_infisso,
);

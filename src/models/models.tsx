export interface IInfisso {
    id: string;
    id_edificio: string;
    tipo: string;
    altezza: number;
    larghezza: number;
    materiale: string;
    vetro: string;
}

export interface IStanza {
    id: number;
    edificio_id: string;
    piano: string;
    id_spazio: string;
    cod_stanza: string;
    destinazione_uso: string;
    altezza?: number,
    spessore_muro?: number,
    riscaldamento?: string,
    raffrescamento?: string,
    illuminazione?: string,
    infissi?: string[];
}

export interface IEdificio {
    chiave: string,
    fascicolo: number,
    indirizzo: string,
    anno_costruzione?: number,
    anno_riqualificazione?: number,
    note_riqualificazione?: string,
    isolamento_tetto?: boolean,
    cappotto?: boolean,
}

export interface IStanzaConInfissi {
    id_stanza: number;
    ids_infissi: string[];
}

export interface VetroInfisso {
    vetro: string;
    eff_energetica: number;
}

export interface MaterialeInfisso {
    materiale: string;
    eff_energetica: number;
}

export interface Illuminazione {
    lampadina: string;
    eff_energetica: number;
}

export interface Climatizzazione {
    climatizzazione: string;
    eff_energetica: number;
}

export interface TipoInfisso {
    nome: string;
}

export interface NuovoTipo {
    tipo: TipoKey;
    name: string;
    eff_energetica: number;
}

export type TipoKey = "riscaldamento" | "raffrescamento" | "illuminazione" | "climatizzazione";


export interface IUtenza {
    id: number;
    id_edificio: string;
    tipo: string;
    cod_contatore: string;
    indirizzo_contatore?: string;
}

export interface IFotovoltaico {
    id: number;
    id_edificio: string;
    potenza: number,
    proprietario: string,
}

type PrimaryKey = | { Edificio: string } | { Stanza: number } | { Infisso: [string, string] }

export interface IAnnotazione {
    id: number,
    ref_table: "edificio" | "stanza" | "infisso",
    id_ref_table: PrimaryKey,
    content: string,
}
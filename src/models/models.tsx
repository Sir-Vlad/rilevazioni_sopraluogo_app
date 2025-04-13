export interface IInfisso {
    id?: string;
    tipo: string;
    altezza: number;
    larghezza: number;
    materiale: string;
    vetro: string;
}

export interface IStanza {
    id: number;
    chiave: string;
    piano: string;
    id_spazio: string;
    stanza: string;
    destinazione_uso: string;
    altezza?: number,
    spessore_muro?: number,
    riscaldamento?: string,
    raffrescamento?: string,
    illuminazione?: string,
    infissi?: string[];
}

export interface IStanzaConInfissi {
    id_stanza: number;
    ids_infissi: string[];
}

export interface VetroInfisso {
    vetro: string;
    efficienza_energetica: number;
}

export interface MaterialeInfisso {
    materiale: string;
    efficienza_energetica: number;
}

export interface Illuminazione {
    lampadina: string;
    efficienza_energetica: number;
}

export interface Climatizzazione {
    climatizzazione: string;
    efficienza_energetica: number;
}
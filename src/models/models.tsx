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
    fascicolo: string;
    piano: string;
    id_spazio: string;
    stanza: string;
    destinazione_uso: string;
    cappotto?: boolean;
    altezza?: number,
    spessore_muro?: number,
    riscaldamento?: string,
    raffrescamento?: string,
    illuminazione?: string,
}
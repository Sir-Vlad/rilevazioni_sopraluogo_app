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
    altezza?: number,
    spessoreMuro?: number,
    riscaldamento?: string,
    raffreddamento?: string,
    illuminazione?: string,
}
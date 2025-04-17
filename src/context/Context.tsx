import { createContext }                from "react";
import { IEdificio, IInfisso, IStanza } from "../models/models.tsx";

export interface DatabaseContextType {
    databasePath: string;
    databaseName: string;
    isLoading: boolean;
    error: string | null;
    needReload: boolean;
    changeDatabase: (dbName: string) => Promise<void>;
    registerProvider: (providerId: string) => { notifyReloadComplete: () => void };
}

export const DatabaseContext = createContext<DatabaseContextType | null>(null);

export interface InfissiContextType {
    data: IInfisso[];
    insertInfisso: (newInfisso: IInfisso) => Promise<void>;
}

export const InfissiContext = createContext<InfissiContextType | null>(null);

export interface IStanzaContext {
    data: IStanza[];
    updateStanza: (newStanza: IStanza) => void;
}

export const StanzeContext = createContext<IStanzaContext | null>(null);

export interface TypeContextType {
    materialiInfissiType: string[];
    vetroInfissiType: string[];
    climatizzazioneType: string[];
    illuminazioneType: string[];
    isLoading: boolean;
}

export const TypesContext = createContext<TypeContextType | null>(null);

export interface EdificioContextType {
    data: IEdificio[];
    selectedEdificio: string | undefined;
    setSelectedEdificio: (chiave: string) => void;
    error: string | null;
    isLoading: boolean;
}

export const EdificioContext = createContext<EdificioContextType | null>(null);
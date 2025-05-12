import { useContext } from "react";
import {
    DatabaseContext,
    EdificioContext,
    FotovoltaicoContext,
    InfissiContext,
    StanzeContext,
    TypesContext,
    UtenzeContext
} from "./Context.tsx";

export const useDatabase = () => {
    const context = useContext(DatabaseContext);
    if (!context) {
        throw new Error("useProvider must be used within the DatabaseProvider");
    }
    return context;
};

export const useInfissi = () => {
    const infissi = useContext(InfissiContext);
    if (!infissi) {
        throw new Error("useInfissi must be used within an InfissiProvider");
    }
    return infissi;
};

export const useTypes = () => {
    const type = useContext(TypesContext);
    if (!type) {
        throw new Error("useType must be used within the DatabaseProvider");
    }
    return type;
};

export const useStanze = () => {
    const stanze = useContext(StanzeContext);
    if (!stanze) {
        throw new Error("useStanze must be used within the DatabaseProvider");
    }
    return stanze;
};

export const useEdifici = () => {
    const edificio = useContext(EdificioContext);
    if (!edificio) {
        throw new Error("useEdificio must be used within the DatabaseProvider");
    }
    return edificio;
};

export const useUtenze = () => {
    const utenze = useContext(UtenzeContext);
    if (!utenze) {
        throw new Error("useUtenze must be used within the DatabaseProvider");
    }
    return utenze;
}

export const useFotovoltaico = () => {
    const utenze = useContext(FotovoltaicoContext);
    if (!utenze) {
        throw new Error("useUtenze must be used within the DatabaseProvider");
    }
    return utenze;
}
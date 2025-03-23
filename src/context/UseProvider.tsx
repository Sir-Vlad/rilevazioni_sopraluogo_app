import { useContext }                                                                            from "react";
import { DatabaseContext, InfissiContext, StanzeConInfissiContext, StanzeContext, TypesContext } from "./Context.tsx";

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

export const useStanzeConInfissi = () => {
    const context = useContext(StanzeConInfissiContext);
    if (!context) {
        throw new Error("useStanzeConInfissi must be defined");
    }
    return context;
};

export const useStanze = () => {
    const stanze = useContext(StanzeContext);
    if (!stanze) {
        throw new Error("useStanze must be used within the DatabaseProvider");
    }
    return stanze;
};
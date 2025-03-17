import { useContext }      from "react";
import { DatabaseContext } from "./DatabaseProvider.tsx";

export const useDatabase = () => {
    const context = useContext(DatabaseContext);
    if (!context) {
        throw new Error("useDatabase must be used within the DatabaseProvider");
    }
    return context;
};
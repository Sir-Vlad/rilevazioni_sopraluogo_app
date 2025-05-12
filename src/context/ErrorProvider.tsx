import { createContext, ReactNode, useCallback, useContext, useMemo, useState } from "react";

// Tipo per l'errore
interface ErrorType {
    id: string;
    message: string;
}

// Tipo per il contesto degli errori
interface ErrorContextType {
    errors: ErrorType[];
    addError: (message: string) => void;
    removeError: (id: string) => void;
    clearErrors: () => void;
}

// Creazione del contesto
const ErrorContext = createContext<ErrorContextType | null>(null);

// Hook per accedere facilmente al contesto
export const useErrorContext = (): ErrorContextType => {
    const context = useContext(ErrorContext);
    if (!context) {
        throw new Error("useErrorContext deve essere usato all'interno di ErrorProvider");
    }
    return context;
};

// Componente provider
export const ErrorProvider = ({ children }: { children: ReactNode }) => {
    const [ errors, setErrors ] = useState<ErrorType[]>([]);

    // Funzione per aggiungere un errore
    const addError = useCallback((message: string) => {
        setErrors((prevErrors) => {
            const exists = prevErrors.some((error) => error.message === message);
            if (exists) {
                return prevErrors;
            }
            const id = Date.now().toString();
            return [ ...prevErrors, {
                id,
                message
            }
            ];
        });
    }, []);

    // Funzione per rimuovere un errore
    const removeError = useCallback((id: string) => {
        setErrors((prevErrors) => prevErrors.filter((error) => error.id !== id));
    }, []);

    // Funzione per svuotare tutti gli errori
    const clearErrors = useCallback(() => {
        setErrors([]);
    }, []);

    const value: ErrorContextType = useMemo(() => ({
        errors,
        addError,
        removeError,
        clearErrors
    }), [ errors, addError, removeError, clearErrors ]);

    return (<ErrorContext.Provider value={ value }>
        { children }
    </ErrorContext.Provider>);
};
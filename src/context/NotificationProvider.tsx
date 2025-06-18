import { createContext, ReactNode, useCallback, useContext, useMemo, useState } from "react";

type TypeMessage = "error" | "warning" | "success";

// Tipo per l'errore
interface MessageItem {
    id: string;
    message: string;
    type: TypeMessage;
}

// Tipo per il contesto degli errori
interface NotificationContextType {
    messageList: MessageItem[];
    addNotification: (message: string, type: TypeMessage) => void;
    removeNotification: (id: string) => void;
    resetNotifications: () => void;
}

// Creazione del contesto
const NotificationContext = createContext<NotificationContextType | null>(null);

// Hook per accedere facilmente al contesto
export const useNotification = (): NotificationContextType => {
    const context = useContext(NotificationContext);
    if (!context) {
        throw new Error("useNotification deve essere usato all'interno di NotificationProvider");
    }
    return context;
};

// Funzione per generare ID univoci con prefisso e counter
function generateUniqueId(): string {
    const prefix = 'msg';
    const timestamp = Date.now();
    const random = Math.floor(Math.random() * 10000);
    return `${prefix}-${timestamp}-${random}`;
}

// Componente provider
export const NotificationProvider = ({ children }: { children: ReactNode }) => {
    const [ notification, setNotification ] = useState<MessageItem[]>([]);

    // Funzione per aggiungere un errore
    const addNotification = useCallback((message: string, type: TypeMessage) => {
        setNotification((prevErrors) => {
            const exists = prevErrors.some((error) => error.message === message);
            if (exists) {
                return prevErrors;
            }
            const id = generateUniqueId();
            return [ ...prevErrors, {
                id,
                message,
                type
            } ];
        });
    }, []);

    // Funzione per rimuovere un errore
    const removeNotification = useCallback((id: string) => {
        setNotification((prevErrors) => prevErrors.filter((error) => error.id !== id));
    }, []);

    // Funzione per svuotare tutti gli errori
    const resetNotification = useCallback(() => {
        setNotification([]);
    }, []);

    const value: NotificationContextType = useMemo(() => ({
        messageList     : notification,
        addNotification   : addNotification,
        removeNotification: removeNotification,
        resetNotifications: resetNotification
    }), [ notification, addNotification, removeNotification, resetNotification ]);

    return (<NotificationContext.Provider value={ value }>
        { children }
    </NotificationContext.Provider>);
};
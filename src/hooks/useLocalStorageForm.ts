import { useEffect } from "react";
import { z } from "zod";
import { UseFormReturn } from "react-hook-form";

/**
 * Hook generico per salvare i dati del form nel localStorage
 *
 * @param form Istanza di useForm di react-hook-form
 * @param key Chiave per il localStorage
 */
export function useLocalStorageForm<T extends z.ZodTypeAny>(form: UseFormReturn<z.infer<T>>, key: string,): void {
    // Salva i dati nel localStorage quando cambiano
    useEffect(() => {
        const subscription = form.watch((value) => {
            localStorage.setItem(key, JSON.stringify(value));
        });
        return () => subscription.unsubscribe();
    }, [ form, key ]);
}

/**
 * Recupera i dati salvati dal localStorage
 *
 * @param key Chiave per il localStorage
 */
export function getSavedFormData<T extends z.ZodTypeAny>(key: string): z.infer<T> | null {
    try {
        const data = localStorage.getItem(key);
        if (!data) return null;

        // eslint-disable-next-line @typescript-eslint/no-unsafe-return
        return JSON.parse(data);
    } catch (error) {
        console.error(`Errore nel recupero dei dati dal localStorage per ${ key }:`, error);
        return null;
    }
}

/**
 * Elimina i dati salvati dal localStorage
 *
 * @param key Chiave per il localStorage
 */
export function clearSavedFormData(key: string): void {
    localStorage.removeItem(key);
}
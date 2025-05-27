import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar.tsx";
import SiteHeader from "@/components/app-header.tsx";
import { AppSidebar } from "@/components/app-sidebar.tsx";
import { Route, Routes } from "react-router-dom";
import PageDashboard from "./pages/DashboardPage/page-dashboard.tsx";
import PageInserimentoData from "@/pages/InserimentoDataPage/page-inserimento-data.tsx";
import Panoramica from "@/pages/page-panoramica.tsx";
import { ThemeProvider } from "./theme/theme-provider";
import { Toaster } from "@/components/ui/sonner.tsx";
import { useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNotification } from "@/context/NotificationProvider.tsx";
import { toast } from "sonner";

function App() {
    const notificationContext = useNotification();
    const processedMessages = useRef<Set<string>>(new Set());

    useEffect(() => {
        // Mostra toast solo per i nuovi messaggi che non sono stati ancora processati
        notificationContext.messageList.forEach(value => {
            if (!processedMessages.current.has(value.id)) {
                processedMessages.current.add(value.id);
                switch (value.type) {
                    case "error":
                        console.error(value.message);
                        toast.error(value.message);
                        break;
                    case "success":
                        toast.success(value.message);
                        break;
                    case "warning":
                        toast.warning(value.message);
                }
            }
        });

        // Rimuovi gli ID di messaggi che non esistono più
        const currentIds = new Set(notificationContext.messageList.map(msg => msg.id));
        processedMessages.current.forEach(id => {
            if (!currentIds.has(id)) {
                processedMessages.current.delete(id);
            }
        });
    }, [notificationContext.messageList]);

    useEffect(() => {
        const cleanupStorage = () => {
            localStorage.clear();
            sessionStorage.clear();

            indexedDB.databases?.().then((databases) => {
                databases?.forEach((db) => {
                    if (db.name) indexedDB.deleteDatabase(db.name);
                });
            }).catch(console.error);
        }

        const closeDatabaseHandler = async () => {
            try {
                await invoke("close_database");
                console.log("Chiusura database ....");
            } catch (e) {
                console.error("Errore durante la chiusura del database: ", e);
            }
        };

        const handleBeforeUnload = async () => {
            cleanupStorage();
            await closeDatabaseHandler();
        }

        window.addEventListener("beforeunload", handleBeforeUnload);

        return () => {
            window.removeEventListener("beforeunload", handleBeforeUnload);
        };
    }, []);

    return <ThemeProvider>
        <div className="[--header-height:calc(theme(spacing.14))]">
            <SidebarProvider className="flex flex-col" defaultOpen={ false }>
                <SiteHeader/>
                <div className="flex flex-1">
                    <AppSidebar/>
                    <SidebarInset>
                        <Routes>
                            <Route path="/" element={ <PageDashboard/> }/>
                            <Route path="/inserimento" element={ <PageInserimentoData/> }/>
                            <Route path="/panoramica" element={ <Panoramica/> }/>
                        </Routes>
                        <Toaster richColors expand={ true } closeButton/>
                    </SidebarInset>
                </div>
            </SidebarProvider>
        </div>
    </ThemeProvider>;
}

export default App;

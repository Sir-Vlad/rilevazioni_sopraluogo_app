import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar.tsx";
import SiteHeader                        from "@/components/app-header.tsx";
import { AppSidebar }                    from "@/components/app-sidebar.tsx";
import { Route, Routes }                 from "react-router-dom";
import PageDashboard                     from "./pages/DashboardPage/page-dashboard.tsx";
import PageInserimentoData               from "@/pages/InserimentoDataPage/page-inserimento-data.tsx";
import Panoramica                        from "@/pages/page-panoramica.tsx";
import { ThemeProvider }                 from "./theme/theme-provider";
import { Toaster }                       from "@/components/ui/sonner.tsx";
import { useEffect }                     from "react";
import { invoke }                        from "@tauri-apps/api/core";
import { useErrorContext }               from "@/context/ErrorProvider.tsx";
import { toast }                         from "sonner";


function App() {
    const errorContext = useErrorContext();

    useEffect(() => {
        errorContext.errors.forEach(value => toast.error(value.message))
    }, [ errorContext.errors ]);

    useEffect(() => {
        const handleBeforeUnload = async () => {
            try {
                await invoke("close_database");
                console.log("Chiusura database ....");
            } catch (e) {
                console.error("Errore durante la chiusura del database: ", e);
            }
        };

        window.addEventListener("beforeunload", handleBeforeUnload);

        return () => {
            window.removeEventListener("beforeunload", handleBeforeUnload);
        };
    }, []);

    return <ThemeProvider>
        <div className="[--header-height:calc(theme(spacing.14))]">
            <SidebarProvider className="flex flex-col" defaultOpen={ false }>
                <SiteHeader />
                <div className="flex flex-1">
                    <AppSidebar />
                    <SidebarInset>
                        <Routes>
                            <Route path="/" element={ <PageDashboard /> } />
                            <Route path="/inserimento" element={ <PageInserimentoData /> } />
                            <Route path="/panoramica" element={ <Panoramica /> } />
                        </Routes>
                        <Toaster richColors expand={ true } closeButton />
                    </SidebarInset>
                </div>
            </SidebarProvider>
        </div>
    </ThemeProvider>;
}

export default App;

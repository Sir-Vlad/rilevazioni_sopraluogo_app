import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar.tsx";
import SiteHeader                        from "@/components/app-header.tsx";
import { AppSidebar }                    from "@/components/app-sidebar.tsx";
import { Route, Routes }                 from "react-router-dom";
import Dashboard                         from "./pages/DashboardPage/Dashboard";
import PageInserimentoData               from "@/pages/InserimentoDataPage/page-inserimento-data.tsx";
import Panoramica                        from "@/pages/page-panoramica.tsx";
import { ThemeProvider }                 from "./theme/theme-provider";
import { Toaster }                       from "@/components/ui/sonner.tsx";


function App() {
    return <ThemeProvider>
        <div className="[--header-height:calc(theme(spacing.14))]">
            <SidebarProvider className="flex flex-col" defaultOpen={ false }>
                <SiteHeader />
                <div className="flex flex-1">
                    <AppSidebar />
                    <SidebarInset>
                        <Routes>
                            <Route path="/" element={ <Dashboard /> } />
                            <Route path="/inserimento" element={ <PageInserimentoData /> } />
                            <Route path="/panoramica" element={ <Panoramica /> } />
                        </Routes>
                        <Toaster richColors expand={ true } />
                    </SidebarInset>
                </div>
            </SidebarProvider>
        </div>
    </ThemeProvider>;
}

export default App;

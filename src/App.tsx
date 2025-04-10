import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar.tsx";
import SiteHeader                        from "@/components/app-header.tsx";
import { AppSidebar }                    from "@/components/app-sidebar.tsx";
import { Route, Routes }                 from "react-router-dom";
import Dashboard                         from "./pages/DashboardPage/Dashboard";
import InserimentoData                   from "@/pages/InserimentoDataPage/InserimentoData.tsx";
import Panoramica                        from "@/pages/Panoramica.tsx";


function App() {
    return <div className="[--header-height:calc(theme(spacing.14))]">
        <SidebarProvider className="flex flex-col">
            <SiteHeader />
            <div className="flex flex-1">
                <AppSidebar />
                <SidebarInset>
                    <Routes>
                        <Route path="/" element={ <Dashboard /> } />
                        <Route path="/inserimento" element={ <InserimentoData /> } />
                        <Route path="/panoramica" element={ <Panoramica /> } />
                    </Routes>
                </SidebarInset>
            </div>
        </SidebarProvider>
    </div>;
}

export default App;

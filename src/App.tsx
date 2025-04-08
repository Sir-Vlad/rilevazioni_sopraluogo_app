import { BrowserRouter, Route, Routes } from "react-router-dom";
import InserimentoData                  from "./pages/InserimentoDataPage/InserimentoData.tsx";
import Panoramica                       from "./pages/Panoramica.tsx";
import Header                           from "./components/Header.tsx";
import Sidebar                          from "./components/Sidebar.tsx";
import GlobalProvider                   from "./context/GlobalProvider.tsx";
import Dashboard                        from "./pages/DashboardPage/Dashboard.tsx";


function App() {
    return <GlobalProvider>
        <BrowserRouter>
            <div className="flex flex-col h-screen w-full">
                {/* Header */ }
                <Header />
                {/* Contenuto principale */ }
                <div className="flex flex-1 overflow-hidden">
                    {/* Sidebar */ }
                    <Sidebar />
                    <Routes>
                        <Route path="/" element={ <Dashboard /> } />
                        <Route path="/inserimento" element={ <InserimentoData /> } />
                        <Route path="/panoramica" element={ <Panoramica /> } />
                    </Routes>
                </div>
            </div>
        </BrowserRouter>
    </GlobalProvider>;
}

export default App;

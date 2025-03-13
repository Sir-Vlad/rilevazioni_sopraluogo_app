import { BrowserRouter, Route, Routes } from "react-router-dom";
import InserimentoData                  from "./pages/InserimentoDataPage/InserimentoData.tsx";
import Panoramica                       from "./pages/Panoramica.tsx";
import Header                           from "./components/Header.tsx";
import Sidebar                          from "./components/Sidebar.tsx";
import DatabaseProvider                 from "./context/DatabaseProvider.tsx";
import TypesProvider                    from "./context/TypesProvider.tsx";
import InfissiProvider                  from "./context/InfissiProvider.tsx";

function App() {
    return <DatabaseProvider>
        <TypesProvider>
            <InfissiProvider>
                <BrowserRouter>
                    <div className="flex flex-col h-screen w-full">
                        {/* Header */ }
                        <Header />
                        {/* Contenuto principale */ }
                        <div className="flex flex-1 overflow-hidden">
                            {/* Sidebar */ }
                            <Sidebar />
                            <Routes>
                                <Route path="/" element={ <InserimentoData /> } />
                                <Route path="/panoramica" element={ <Panoramica /> } />
                            </Routes>
                        </div>
                    </div>
                </BrowserRouter>
            </InfissiProvider>
        </TypesProvider>
    </DatabaseProvider>;
}

export default App;

import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import GlobalProvider from "./context/GlobalProvider.tsx";
import { BrowserRouter } from "react-router-dom";

createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <GlobalProvider>
            <BrowserRouter>
                <App/>
            </BrowserRouter>
        </GlobalProvider>
    </StrictMode>
);

import * as React from "react";
import DatabaseProvider from "./DatabaseProvider.tsx";
import TypesProvider from "./TypesProvider.tsx";
import InfissiProvider from "./InfissiProvider.tsx";
import StanzeProvider from "./StanzeProvider.tsx";
import EdificioProvider from "@/context/EdificioProvider.tsx";
import UtenzeProvider from "./UtenzeProvider.tsx";
import FotovoltaicoProvider from "@/context/FotovoltaicoProvider.tsx";
import { NotificationProvider } from "@/context/NotificationProvider.tsx";

const GlobalProvider = ({ children }: { children: React.ReactNode }) => {
    return <NotificationProvider>
        <DatabaseProvider>
            <TypesProvider>
                <EdificioProvider>
                    <InfissiProvider>
                        <StanzeProvider>
                            <UtenzeProvider>
                                <FotovoltaicoProvider>
                                    { children }
                                </FotovoltaicoProvider>
                            </UtenzeProvider>
                        </StanzeProvider>
                    </InfissiProvider>
                </EdificioProvider>
            </TypesProvider>
        </DatabaseProvider>
    </NotificationProvider>;
};

export default GlobalProvider;
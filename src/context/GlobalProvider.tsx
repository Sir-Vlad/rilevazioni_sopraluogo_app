import EdificioProvider from "@/context/EdificioProvider.tsx";
import FotovoltaicoProvider from "@/context/FotovoltaicoProvider.tsx";
import {NotificationProvider} from "@/context/NotificationProvider.tsx";
import {SelectedEdificioProvider} from "@/context/SelectedEdificioProvider.tsx";
import * as React from "react";
import DatabaseProvider from "./DatabaseProvider.tsx";
import InfissiProvider from "./InfissiProvider.tsx";
import StanzeProvider from "./StanzeProvider.tsx";
import TypesProvider from "./TypesProvider.tsx";
import UtenzeProvider from "./UtenzeProvider.tsx";

const GlobalProvider = ({children}: { children: React.ReactNode }) => {
    return <NotificationProvider>
        <DatabaseProvider>
            <TypesProvider>
                <EdificioProvider>
                    <SelectedEdificioProvider>
                        <InfissiProvider>
                            <StanzeProvider>
                                <UtenzeProvider>
                                    <FotovoltaicoProvider>
                                        {children}
                                    </FotovoltaicoProvider>
                                </UtenzeProvider>
                            </StanzeProvider>
                        </InfissiProvider>
                    </SelectedEdificioProvider>
                </EdificioProvider>
            </TypesProvider>
        </DatabaseProvider>
    </NotificationProvider>;
};

export default GlobalProvider;
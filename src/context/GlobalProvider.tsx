import * as React               from "react";
import DatabaseProvider         from "./DatabaseProvider.tsx";
import TypesProvider            from "./TypesProvider.tsx";
import InfissiProvider          from "./InfissiProvider.tsx";
import StanzeProvider           from "./StanzeProvider.tsx";
import StanzeConInfissiProvider from "./StanzeConInfissiProvider.tsx";
import EdificioProvider         from "@/context/EdificioProvider.tsx";

const GlobalProvider = ({children}: { children: React.ReactNode }) => {
    return <DatabaseProvider>
        <TypesProvider>
            <EdificioProvider>
                <InfissiProvider>
                    <StanzeProvider>
                        <StanzeConInfissiProvider>
                            { children }
                        </StanzeConInfissiProvider>
                    </StanzeProvider>
                </InfissiProvider>
            </EdificioProvider>
        </TypesProvider>
    </DatabaseProvider>;
};

export default GlobalProvider;
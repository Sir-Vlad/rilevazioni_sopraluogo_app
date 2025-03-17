import * as React       from "react";
import DatabaseProvider from "./DatabaseProvider.tsx";
import TypesProvider    from "./TypesProvider.tsx";
import InfissiProvider  from "./InfissiProvider.tsx";
import StanzeProvider   from "./StanzeProvider.tsx";

const GlobalProvider = ({children}: { children: React.ReactNode }) => {
    return <DatabaseProvider>
        <TypesProvider>
            <InfissiProvider>
                <StanzeProvider>
                    { children }
                </StanzeProvider>
            </InfissiProvider>
        </TypesProvider>
    </DatabaseProvider>;
};

export default GlobalProvider;
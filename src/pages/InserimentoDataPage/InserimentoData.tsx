import Box          from "../../components/Box";
import "./InserimentoData.css";
import FormStanza   from "./FormStanza.tsx";
import FormInfisso  from "./FormInfisso.tsx";
import TableInfissi from "./TableInfissi.tsx";

const InserimentoData = () => {
    return (<main className="flex-1 p-4 bg-gray-400 flex">
        <div className="flex flex-col w-2/4 mr-4">
            <Box className="mb-4 flex-1">
                <FormStanza />
            </Box>
            <Box className="flex-1">
                <FormInfisso />
            </Box>
        </div>

        <div className="w-2/4">
            <Box className="h-full w-full">
                <div className="flex flex-row items-center gap-5 mb-6 h-[90%]">
                    <TableInfissi />
                </div>
            </Box>
        </div>
    </main>);

};

export default InserimentoData;
import Box                from "../../components/Box";
import "./InserimentoData.css";
import FormStanza         from "./FormStanza.tsx";
import FormInfisso        from "./FormInfisso.tsx";
import TableInfissi       from "./TableInfissi.tsx";
import { ToastContainer } from "react-toastify";

const InserimentoData = () => {
    return (<main className="flex-1 p-4 bg-gray-400 flex">
        <div className="w-2/4 mr-4">
            <Box className="mb-4 h-full">
                <FormStanza />
            </Box>
        </div>
        <div className="flex flex-col w-2/4 gap-4 ">
            <Box className="">
                <FormInfisso />
            </Box>
            <Box className="flex-1 w-full">
                <div className="flex flex-row items-center gap-5 mb-6 h-[85%]">
                    <TableInfissi />
                </div>
            </Box>
        </div>
        <ToastContainer />
    </main>);

};

export default InserimentoData;
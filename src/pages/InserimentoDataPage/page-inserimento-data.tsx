import CardFormStanza   from "@/pages/InserimentoDataPage/card-form-stanza.tsx";
import CardFormInfisso  from "@/pages/InserimentoDataPage/card-form-infisso.tsx";
import CardTableInfissi from "@/pages/InserimentoDataPage/card-table-infissi.tsx";

const PageInserimentoData = () => {
    return (<div className="flex flex-1 flex-col">
        <div className="@container/main flex flex-1 flex-col gap-2">
            <div className="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
                <div className="flex flex-row justify-between items-center px-7">
                    <h1 className="text-2xl font-bold text-primary tracking-tight mb-4">Gestione Stanze</h1>
                </div>
                <div className="flex flex-1">
                    <div className="w-2/4 mr-4">
                        <CardFormStanza />
                    </div>
                    <div className="flex flex-col w-2/4 gap-4 ">
                        <CardFormInfisso />
                        <CardTableInfissi />
                    </div>
                </div>
            </div>
        </div>
    </div>);

};

export default PageInserimentoData;
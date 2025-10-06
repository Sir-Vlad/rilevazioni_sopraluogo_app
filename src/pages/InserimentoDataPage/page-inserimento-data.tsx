import TitlePage from "@/components/title-page.tsx";
import CardFormInfisso from "@/pages/InserimentoDataPage/card-form-infisso.tsx";
import CardFormStanza from "@/pages/InserimentoDataPage/card-form-stanza.tsx";
import CardTableInfissi from "@/pages/InserimentoDataPage/card-table-infissi.tsx";

const PageInserimentoData = () => {
    return (
        <div className="flex flex-1 flex-col">
        <div className="@container/main flex flex-1 flex-col">
            <div className="flex flex-col gap-3 py-4 md:gap-4 md:py-6">
                <div className="flex flex-row justify-start items-center px-7 gap-5 mb-4">
                    <TitlePage title={"Gestione Stanze"}/>
                </div>
                <div className="flex flex-1">
                    <div className="w-2/4">
                        <CardFormStanza/>
                    </div>
                    <div className="flex flex-col w-2/4 gap-4">
                        <CardFormInfisso/>
                        <CardTableInfissi/>
                    </div>
                </div>
            </div>
        </div>
    </div>);

};

export default PageInserimentoData;
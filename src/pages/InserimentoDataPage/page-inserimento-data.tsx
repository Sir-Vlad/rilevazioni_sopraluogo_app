import CardFormStanza from "@/pages/InserimentoDataPage/card-form-stanza.tsx";
import CardFormInfisso from "@/pages/InserimentoDataPage/card-form-infisso.tsx";
import CardTableInfissi from "@/pages/InserimentoDataPage/card-table-infissi.tsx";
import TitlePage from "@/components/title-page.tsx";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select.tsx";
import { useEdifici } from "@/context/UseProvider.tsx";
import { Separator } from "@/components/ui/separator";

const PageInserimentoData = () => {
    const edificioContext = useEdifici();
    const edifici = [ ...edificioContext.data.map(value => value.chiave) ];

    return (<div className="flex flex-1 flex-col">
        <div className="@container/main flex flex-1 flex-col">
            <div className="flex flex-col gap-3 py-4 md:gap-4 md:py-6">
                <div className="flex flex-row justify-start items-center px-7 gap-5 mb-4">
                    <div>
                        <TitlePage title={ "Gestione Stanze" }/>
                    </div>
                    <Separator orientation={ "vertical" }/>
                    <div>
                        <Select disabled={ edifici.length < 2 } value={ edificioContext.selectedEdificio }
                                onValueChange={ edificioContext.setSelectedEdificio }
                        >
                            <SelectTrigger className="w-[10em] border-none dark:bg-transparent">
                                <SelectValue placeholder="" defaultValue={ edifici[0] }/>
                            </SelectTrigger>
                            <SelectContent>
                                { edifici.map(value => {
                                    return <SelectItem value={ value } key={ value }>{ value }</SelectItem>;
                                }) }
                            </SelectContent>
                        </Select>
                    </div>
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
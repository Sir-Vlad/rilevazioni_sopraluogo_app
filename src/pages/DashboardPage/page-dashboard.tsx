import { SectionCards } from "@/pages/DashboardPage/section-cards.tsx";
import SectionData from "@/pages/DashboardPage/section-data.tsx";
import SectionProgressGraph from "@/pages/DashboardPage/section-progress-graph.tsx";
import SectionDataEdificio from "@/pages/DashboardPage/section-data-edificio.tsx";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select.tsx";
import TitlePage from "@/components/title-page.tsx";
import { useEdifici } from "@/context/UseProvider.tsx";

const PageDashboard = () => {
    const edificioContext = useEdifici();
    const edifici = [ ...edificioContext.data.map(value => value.chiave) ];

    return <div className="flex flex-1 flex-col">
        <div className="@container/main flex flex-1 flex-col gap-2">
            <div className="flex flex-col gap-3 py-4 md:gap-4 md:py-6">
                <div className="flex flex-row justify-start items-center px-7 gap-5 mb-4">
                    <div>
                        <TitlePage title={ "Dashboard" }/>
                    </div>
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
                <SectionCards/>
                <SectionProgressGraph/>
                <SectionData/>
                <SectionDataEdificio/>
            </div>
        </div>
    </div>;
};

export default PageDashboard;
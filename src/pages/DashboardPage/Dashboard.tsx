import { SectionCards }                                                  from "@/components/section-cards.tsx";
import SectionData                                                       from "@/components/section-data";
import SectionProgressGraph                                              from "@/components/section-progress-graph.tsx";
import SectionDataEdificio                                               from "@/components/section-data-edificio.tsx";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select.tsx";

const dataFake = [ "4587-45", "8455-12" ];

const Dashboard = () => {
    return <div className="flex flex-1 flex-col">
        <div className="@container/main flex flex-1 flex-col gap-2">
            <div className="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
                <div className="flex flex-row justify-start items-center px-7 mb-4 gap-5">
                    <div>
                        <h1 className="text-2xl font-bold text-primary tracking-tight">Dashboard</h1>
                    </div>
                    <div>
                        <Select>
                            <SelectTrigger className="w-[10em] border-none dark:bg-transparent">
                                <SelectValue placeholder="" />
                            </SelectTrigger>
                            <SelectContent>
                                { dataFake.map(value => {
                                    return <SelectItem value={ value } key={ value }>{ value }</SelectItem>;
                                }) }
                            </SelectContent>
                        </Select>
                    </div>
                </div>
                <SectionCards />
                <SectionProgressGraph />
                <SectionData />
                <SectionDataEdificio />
            </div>
        </div>
    </div>;
};

export default Dashboard;
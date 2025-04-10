import { SectionCards }     from "@/components/section-cards.tsx";
import SectionData          from "@/components/section-data";
import SectionProgressGraph from "@/components/section-progress-graph.tsx";

const Dashboard = () => {
    return <div className="flex flex-1 flex-col">
        <div className="@container/main flex flex-1 flex-col gap-2">
            <div className="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
                <div className="flex flex-row justify-between items-center px-7">
                    <h1 className="text-2xl font-bold text-primary tracking-tight mb-4">Dashboard</h1>
                </div>
                <SectionCards />
                <SectionData />
                <SectionProgressGraph />
                {/*<div className="px-4 lg:px-6">*/ }
                {/*    <ChartAreaInteractive />*/ }
                {/*</div>*/ }
                {/*<DataTable data={ data } />*/ }
            </div>
        </div>
    </div>;
};

export default Dashboard;
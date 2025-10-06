import TitlePage from "@/components/title-page.tsx";
import {SectionCards} from "@/pages/DashboardPage/section-cards.tsx";
import SectionDataEdificio from "@/pages/DashboardPage/section-data-edificio.tsx";
import SectionData from "@/pages/DashboardPage/section-data.tsx";
import SectionProgressGraph from "@/pages/DashboardPage/section-progress-graph.tsx";

const PageDashboard = () => {
    return <div className="flex flex-1 flex-col">
        <div className="@container/main flex flex-1 flex-col gap-2">
            <div className="flex flex-col gap-3 py-4 md:gap-4 md:py-6">
                <div className="flex flex-row justify-start items-center px-7 gap-5 mb-4">
                    <TitlePage title={"Dashboard"}/>
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
import CardTableStanze  from "@/pages/DashboardPage/card-table-stanze.tsx";
import CardDataEdificio from "@/pages/DashboardPage/card-data-edificio.tsx";

const SectionData = () => {
    return <div className="*:data-[slot=card]:shadow-xs grid grid-cols-8 gap-4
            px-4 *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card
            dark:*:data-[slot=card]:bg-card lg:px-6">
        <CardTableStanze />
        <CardDataEdificio />
    </div>;
};

export default SectionData;
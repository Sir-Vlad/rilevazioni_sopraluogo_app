import CardProgress from "@/components/card-progress.tsx";

const dataMateriali = {
    title : "Materiali",
    values: [
        {
            label: "PVC",
            value: 20
        },
        {
            label: "Legno",
            value: 50
        },
        {
            label: "Alluminio",
            value: 30
        }
    ]
};

const dataVetro = {
    title : "Vetro",
    values: [
        {
            label: "Singolo",
            value: 45
        },
        {
            label: "Doppio",
            value: 35
        },
        {
            label: "Triplo",
            value: 20
        }
    ]
};

const SectionProgressGraph = () => {
    return <div className="*:data-[slot=card]:shadow-xs grid grid-cols-2 gap-4
            px-4 *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card
            dark:*:data-[slot=card]:bg-card lg:px-6">
        <CardProgress title={ dataMateriali.title } values={ dataMateriali.values } />
        <CardProgress title={ dataVetro.title } values={ dataVetro.values } />
    </div>;
};

export default SectionProgressGraph;
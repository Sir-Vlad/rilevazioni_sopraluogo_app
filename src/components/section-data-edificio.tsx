import CardUtenzeEdificio from "@/components/card-utenze-edificio.tsx";
import CardFotovoltaico   from "@/components/card-fotovoltaico.tsx";

const SectionDataEdificio = () => {
    return <div className="*:data-[slot=card]:shadow-xs grid grid-cols-2 gap-4
            px-4 *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card
            dark:*:data-[slot=card]:bg-card lg:px-6">
        <CardUtenzeEdificio />
        <CardFotovoltaico />
    </div>;
};

export default SectionDataEdificio;
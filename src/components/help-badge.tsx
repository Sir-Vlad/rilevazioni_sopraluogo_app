import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip.tsx";
import { HelpCircle } from "lucide-react";

const HelpBadge = ({ message }: { message: string }) => {
    return <div tabIndex={ -1 }>
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger asChild>
                    <HelpCircle className="w-4 h-4 text-red-700"/>
                </TooltipTrigger>
                <TooltipContent side="right">
                    <p className="text-white">{ message }</p>
                </TooltipContent>
            </Tooltip>
        </TooltipProvider>
    </div>;
};

export default HelpBadge;
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger
}                                                                   from "@/components/ui/dialog.tsx";
import { Button }                                                   from "@/components/ui/button.tsx";
import { PlusIcon }                                                 from "lucide-react";
import { Textarea }                                                 from "@/components/ui/textarea.tsx";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip.tsx";

const CommentButton = () => {
    return <TooltipProvider>
        <Tooltip>
            <Dialog>
                <TooltipTrigger asChild>
                    <DialogTrigger asChild>
                        <Button variant="outline" size="icon" type="button">
                            <PlusIcon />
                        </Button>
                    </DialogTrigger>
                </TooltipTrigger>
                <DialogContent className="sm:max-w-md">
                    <DialogHeader>
                        <DialogTitle>Inserisci Commento</DialogTitle>
                        <DialogDescription>Inserisci il commento</DialogDescription>
                    </DialogHeader>
                    <div className="flex flex-col gap-2">
                        <Textarea rows={ 5 } placeholder="Inserisci il commento" />
                    </div>
                    <DialogFooter className="sm:justify-end">
                        <DialogClose asChild>
                            <Button type="button" className="text-white">
                                <PlusIcon /><span>Inserisci Commento</span>
                            </Button>
                        </DialogClose>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
            <TooltipContent side="right">
                <p className="text-white">Aggiungi un commento</p>
            </TooltipContent>
        </Tooltip>
    </TooltipProvider>;
};

export default CommentButton;
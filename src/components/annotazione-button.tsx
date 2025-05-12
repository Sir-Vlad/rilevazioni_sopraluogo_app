import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger
} from "@/components/ui/dialog.tsx";
import { Button } from "@/components/ui/button.tsx";
import { PlusIcon } from "lucide-react";
import { Textarea } from "@/components/ui/textarea.tsx";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip.tsx";
import { Dispatch, SetStateAction, useState } from "react";

interface AnnotazioneButtonProps {
    setAnnotazione: Dispatch<SetStateAction<string[]>>
}


const AnnotazioneButton = ({
                               setAnnotazione
                           }: AnnotazioneButtonProps) => {
    const [ content, setContent ] = useState<string | null>(null);

    return <TooltipProvider>
        <Tooltip>
            <Dialog>
                <TooltipTrigger asChild>
                    <DialogTrigger asChild>
                        <Button variant="outline" size="icon" type="button">
                            <PlusIcon/>
                        </Button>
                    </DialogTrigger>
                </TooltipTrigger>
                <DialogContent className="sm:max-w-xl">
                    <DialogHeader>
                        <DialogTitle>Inserisci una annotazione</DialogTitle>
                        <DialogDescription>Inserisci una annotazione</DialogDescription>
                    </DialogHeader>
                    <div className="flex flex-col gap-2">
                        <Textarea rows={ 5 }
                                  placeholder="Inserisci l'annotazione qui..."
                                  value={ content ?? "" }
                                  onChange={ (e) => setContent(e.target.value) }
                        />
                    </div>
                    <DialogFooter className="sm:justify-end">
                        <DialogClose asChild>
                            <Button type="button" className="text-white"
                                    onClick={ () => {
                                        if (content === null) return;
                                        setAnnotazione((prev) => [ ...prev, content ]);
                                        setContent(null);
                                    } }>
                                <PlusIcon/><span>Inserisci Annotazione</span>
                            </Button>
                        </DialogClose>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
            <TooltipContent side="right">
                <p className="text-white">Aggiungi una annotazione</p>
            </TooltipContent>
        </Tooltip>
    </TooltipProvider>;
};

export default AnnotazioneButton;
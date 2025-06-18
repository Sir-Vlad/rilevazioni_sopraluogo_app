import { Sheet, SheetContent, SheetDescription, SheetHeader, SheetTitle, SheetTrigger, } from "@/components/ui/sheet"
import { Button } from "@/components/ui/button.tsx";
import { AlertCircle, Bell, CheckCircle, Trash, XCircle } from "lucide-react";
import { useNotification } from "@/context/NotificationProvider.tsx";

const NotificationSidebar = () => {
    const {
        messageList,
        resetNotifications,
        removeNotification
    } = useNotification();

    return (<Sheet>
        <SheetTrigger asChild>
            <Button variant="ghost" size="icon">
                <Bell className="h-5 w-5"/>
            </Button>
        </SheetTrigger>
        <SheetContent side="right" className="w-[600px] sm:max-w-screen gap-3">
            <SheetHeader className="py-4">
                <SheetTitle>Notifiche</SheetTitle>
                <SheetDescription>
                    Qui puoi visualizzare tutte le tue notifiche recenti.
                </SheetDescription>
            </SheetHeader>
            <div className="flex justify-end px-4">
                <Button variant="ghost" size="lg" onClick={ resetNotifications }>
                    <Trash/> Cancella notifiche
                </Button>
            </div>
            <div className="mx-1 p-4 h-full overflow-y-auto flex flex-col gap-2">
                {/* Contenuto delle notifiche qui */ }
                { messageList.length > 0 ? messageList.map(({
                                                                id,
                                                                message,
                                                                type
                                                            }) => {
                    return <NotificationBox key={ id } type={ type } message={ message }
                                            onRemove={ () => removeNotification(id) }/>
                }) : <p>Non hai nuove notifiche.</p> }
            </div>
        </SheetContent>
    </Sheet>)


}

interface NotificationBoxProps {
    type: "error" | "success" | "warning";
    message: string;
    onRemove?: () => void;
}

const NotificationBox = ({
                             type,
                             message,
                             onRemove
                         }: NotificationBoxProps) => {

    switch (type) {
        case "error":
            return <div className="flex justify-between items-center gap-2 p-2 border-2 border-red-400 rounded-md">
                <div className="flex items-center gap-2">
                    <XCircle className="text-red-400"/>{ message }
                </div>
                <div>
                    <Button type="button" variant="ghost" size="sm" onClick={ onRemove }>
                        <Trash/>
                    </Button>
                </div>
            </div>
        case "success":
            return <div className="flex justify-between items-center gap-2 p-2 border-2 border-green-400 rounded-md">
                <div className="flex items-center gap-2">
                    <CheckCircle className="text-green-400"/>{ message }
                </div>
                <div>
                    <Button type="button" variant="ghost" size="sm" onClick={ onRemove }>
                        <Trash/>
                    </Button>
                </div>
            </div>
        case "warning":
            return <div className="flex justify-between items-center gap-2 p-2 border-2 border-yellow-400 rounded-md">
                <div className="flex items-center gap-2">
                    <AlertCircle className="text-yellow-400"/>{ message }
                </div>
                <div>
                    <Button type="button" variant="ghost" size="sm" onClick={ onRemove }>
                        <Trash/>
                    </Button>
                </div>
            </div>
    }
}


export default NotificationSidebar;
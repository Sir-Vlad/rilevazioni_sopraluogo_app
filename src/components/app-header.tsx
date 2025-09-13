import {ModeToggle} from "@/components/mode-toggle.tsx";
import NotificationSidebar from "@/components/notification-sidebar.tsx";
import {Button} from "@/components/ui/button";
import {
    NavigationMenu,
    NavigationMenuItem,
    NavigationMenuLink,
    NavigationMenuList,
    navigationMenuTriggerStyle
} from "@/components/ui/navigation-menu.tsx";
import {Separator} from "@/components/ui/separator.tsx";
import {useSidebar} from "@/components/ui/sidebar";
import {useSelectedEdificio} from "@/context/SelectedEdificioProvider.tsx";
import {SidebarIcon} from "lucide-react";
import {ReactNode, useMemo} from "react";
import {NavLink} from "react-router-dom";

export default function SiteHeader() {
    const {toggleSidebar} = useSidebar();
    const {edificio} = useSelectedEdificio();

    const fileSelected = useMemo((): ReactNode => {
        return edificio?.chiave !== undefined ? <>Stai lavorando sull'edificio <b>{edificio?.chiave}</b></> : <>Selezionare un file</>;
    }, [edificio?.chiave]);

    return (
        <header className="flex sticky top-0 z-50 w-full items-center border-b bg-background">
        <div className="flex h-[var(--header-height)] w-full items-center justify-between gap-2 px-4">
            <div className="w-full h-full flex justify-start items-center">
                <Button
                    className="h-8 w-8"
                    variant="ghost"
                    size="icon"
                    onClick={toggleSidebar}
                >
                    <SidebarIcon/>
                </Button>
                <Separator orientation="vertical" className="mx-2"/>
                <p>Dashboard</p>
            </div>
            <div className="w-full justify-center align-middle">
                <p className="text-sm">
                    {fileSelected}
                </p>
            </div>
            <div className="flex justify-end">
                <NavigationMenu>
                    <NavigationMenuList>
                        <NavigationMenuItem>
                            <NavigationMenuLink className={navigationMenuTriggerStyle()} asChild>
                                <NavLink to="/" end>
                                    Dashboard
                                </NavLink>
                            </NavigationMenuLink>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                            <NavigationMenuLink className={navigationMenuTriggerStyle()} asChild>
                                <NavLink to="/inserimento" end>
                                    Inserimento
                                </NavLink>
                            </NavigationMenuLink>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                            <NavigationMenuLink className={navigationMenuTriggerStyle()} asChild>
                                <NavLink to="/panoramica" end>
                                    Panoramica
                                </NavLink>
                            </NavigationMenuLink>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>
            </div>
            <Separator orientation="vertical" className="mx-1"/>
            <ModeToggle/>
            <NotificationSidebar/>
        </div>
    </header>);
}
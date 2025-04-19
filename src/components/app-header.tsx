import { SidebarIcon } from "lucide-react";
import {
    Button
}                      from "@/components/ui/button";
import {
    useSidebar
}                      from "@/components/ui/sidebar";
import {
    NavLink
}                      from "react-router-dom";
import {
    NavigationMenu,
    NavigationMenuItem,
    NavigationMenuLink,
    NavigationMenuList,
    navigationMenuTriggerStyle
}                      from "@/components/ui/navigation-menu.tsx";
import {
    useDatabase
}                      from "@/context/UseProvider.tsx";
import {
    Separator
}                      from "@/components/ui/separator.tsx";
import {
    ModeToggle
}                      from "@/components/mode-toggle.tsx";
import {
    ReactNode,
    useMemo
}                      from "react";

export default function SiteHeader() {
    const {toggleSidebar} = useSidebar();
    const {databaseName} = useDatabase();

    const fileSelected = useMemo((): ReactNode => {
        return databaseName !== null ? <>Stai lavorando
            sul fascicolo <b>{ Number(databaseName) }</b></> : <>Selezionare un file</>;
    }, [ databaseName ]);

    return (<header className="flex sticky top-0 z-50 w-full items-center border-b bg-background">
        <div className="flex h-[var(--header-height)] w-full items-center justify-between gap-2 px-4">
            <div className="w-full h-full flex justify-start items-center">
                <Button
                    className="h-8 w-8"
                    variant="ghost"
                    size="icon"
                    onClick={ toggleSidebar }
                >
                    <SidebarIcon />
                </Button>
                <Separator orientation="vertical" className="mx-2" />
                <p>Dashboard</p>
            </div>
            <div className="w-full justify-center align-middle">
                <p className="text-sm">
                    { fileSelected }
                </p>
            </div>
            <div className="flex justify-end">
                <NavigationMenu>
                    <NavigationMenuList>
                        <NavigationMenuItem>
                            <NavigationMenuLink className={ navigationMenuTriggerStyle() } asChild>
                                <NavLink to="/" end>
                                    Dashboard
                                </NavLink>
                            </NavigationMenuLink>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                            <NavigationMenuLink className={ navigationMenuTriggerStyle() } asChild>
                                <NavLink to="/inserimento" end>
                                    Inserimento
                                </NavLink>
                            </NavigationMenuLink>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                            <NavigationMenuLink className={ navigationMenuTriggerStyle() } asChild>
                                <NavLink to="/panoramica" end>
                                    Panoramica
                                </NavLink>
                            </NavigationMenuLink>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>
            </div>
            <Separator orientation="vertical" className="mx-2" />
            <ModeToggle />
        </div>
    </header>);
}
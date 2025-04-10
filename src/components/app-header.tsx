import { SidebarIcon } from "lucide-react";
import {
    Button
}                      from "@/components/ui/button";
import {
    useSidebar
}                      from "@/components/ui/sidebar";
import {
    Link
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

export default function SiteHeader() {
    const {toggleSidebar} = useSidebar();
    const {
              databaseName,
              error
          } = useDatabase();

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
                    { error !== "Database non settato" ?
                        <>Stai lavorando su <b>{ databaseName }</b></> :
                        "Selezionare un file"
                    }
                </p>
            </div>
            <div className="flex justify-end">
                <NavigationMenu>
                    <NavigationMenuList>
                        <NavigationMenuItem>
                            <NavigationMenuLink className={ navigationMenuTriggerStyle() }>
                                <Link to="/">Dashboard</Link>
                            </NavigationMenuLink>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                            <NavigationMenuLink className={ navigationMenuTriggerStyle() }>
                                <Link to="/inserimento">Inserimento</Link>
                            </NavigationMenuLink>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                            <NavigationMenuLink className={ navigationMenuTriggerStyle() }>
                                <Link to="/panoramica">Panoramica</Link>
                            </NavigationMenuLink>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>
            </div>
        </div>
    </header>);
}
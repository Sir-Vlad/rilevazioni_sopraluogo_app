import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card.tsx";
import { Label }                                    from "@/components/ui/label.tsx";
import { Progress }                                 from "@/components/ui/progress.tsx";
import TitleCard                                    from "@/components/title-card.tsx";

interface CardProgressProps {
    title: string;
    values: {
        label: string;
        value: number;
    }[];
}


const CardProgress = ({
                          title,
                          values
                      }: CardProgressProps) => {
    const total = values.reduce((acc, value) => acc + value.value, 0);

    return <Card className="@container/card">
        <CardHeader className="relative">
            <CardTitle className="text-2xl font-semibold tabular-nums">
                <TitleCard title={ title } />
            </CardTitle>
        </CardHeader>
        <CardContent>
            <div className="flex flex-col gap-3">
                { values.length > 0 ?
                    (values.map(value => {
                        const valuePercent = total !== 0 ? value.value / total * 100 : 0;
                        return <div
                            className="grid grid-cols-12 items-center align-middle rounded-md border p-4 hover:bg-sidebar-accent/90"
                            key={ value.label }>
                            <Label className="col-span-2">{ value.label }</Label>
                            <Progress className="col-span-9" value={ valuePercent } />
                            <div className="flex justify-end">
                                <Label className="col-span-1">{ Math.round(valuePercent) } %</Label>
                            </div>
                        </div>;
                    })) : (
                        <div className="h-24 flex items-center justify-center rounded-md border p-4">
                            <span>No results</span>
                        </div>
                    )
                }
            </div>
        </CardContent>
    </Card>;
};

export default CardProgress;
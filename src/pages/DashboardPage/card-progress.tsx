import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card.tsx";
import { Label }                                    from "@/components/ui/label.tsx";
import { Progress }                                 from "@/components/ui/progress.tsx";

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
                { title }
            </CardTitle>
        </CardHeader>
        <CardContent>
            <div className="flex flex-col gap-6">
                {
                    values.map(value => {
                        const valuePercent = value.value / total * 100;
                        return <div className="grid grid-cols-12 gap-4 items-center align-middle" key={ value.label }>
                            <Label className="col-span-2">{ value.label }</Label>
                            <Progress className="col-span-9" value={ valuePercent } />
                            <div className="flex justify-end">
                                <Label className="col-span-1">{ Math.round(valuePercent) } %</Label>
                            </div>
                        </div>;
                    })
                }
            </div>
        </CardContent>
    </Card>;
};

export default CardProgress;
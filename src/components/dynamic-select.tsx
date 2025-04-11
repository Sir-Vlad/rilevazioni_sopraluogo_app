import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select.tsx";
import { FormControl }                                                   from "@/components/ui/form.tsx";
import { Button }                                                        from "@/components/ui/button.tsx";
import { PlusIcon }                                                      from "lucide-react";
import { useInfissi }                                                    from "@/context/UseProvider.tsx";

interface DynamicSelectProps {
    onChange: (value: string[]) => void;
    value: string[] | undefined;
}


const DynamicSelect = ({
                           onChange,
                           value
                       }: DynamicSelectProps) => {
    const infissi = useInfissi();
    const infissiData = infissi.data.map((item) => {
        if (!item.id) {
            throw new Error("Infisso doesn't have id");
        }
        return item.id;
    });


    return <div className="grid grid-cols-4 gap-5">
        { (value || []).map((_item, index) => {
            return <div key={ index + 1 }>
                <Select onValueChange={ value1 => {
                    const updateValues = [ ...value || [] ];
                    updateValues[index] = value1;
                    onChange(updateValues);
                } } defaultValue={ value === undefined ? undefined : value[0] }>
                    <FormControl>
                        <SelectTrigger className="w-full">
                            <SelectValue
                                placeholder="" />
                        </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                        {
                            infissiData.map(value1 => {
                                return <SelectItem value={ value1 } key={ value1 }>{ value1 }</SelectItem>;
                            })
                        }
                    </SelectContent>
                </Select>
            </div>;
        }) }

        <div className="flex flex-col gap-2">
            <Button variant="outline" size="icon" type="button" onClick={ () => {
                const newValues = [ ...(value || []), "" ];
                onChange(newValues);
            } }>
                <PlusIcon />
            </Button>
        </div>
    </div>;
};

export default DynamicSelect;
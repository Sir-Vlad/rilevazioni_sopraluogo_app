import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card.tsx";

import { z }                                                              from "zod";
import { useForm }                                                        from "react-hook-form";
import { zodResolver }                                                    from "@hookform/resolvers/zod";
import { Button }                                                         from "@/components/ui/button";
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "@/components/ui/form";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue }  from "@/components/ui/select";
import DynamicSelect                                                      from "@/components/dynamic-select.tsx";
import { Input }                                                          from "@/components/ui/input.tsx";
import { ChangeEvent }                                                    from "react";
import { Pencil }                                                         from "lucide-react";
import CommentButton                                                      from "@/components/comment-button.tsx";
import HelpBadge                                                          from "@/components/help-badge.tsx";


const FormSchema = z.object({
    stanza          : z.string({
        required_error: "Please select an email to display."
    }),
    destinazione_uso: z.string(),
    piano           : z.string(),
    altezza         : z.number().positive(),
    spessore_muro   : z.number().positive(),
    riscaldamento   : z.string().optional(),
    raffrescamento  : z.string().optional(),
    illuminazione   : z.string().optional(),
    infissi         : z.array(z.string()).optional()
});


const CardFormStanza = () => {
    const form = useForm<z.infer<typeof FormSchema>>({
        resolver     : zodResolver(FormSchema),
        defaultValues: {
            altezza      : 0,
            spessore_muro: 0
        }
    });

    const handleInputNumericChange = (event: ChangeEvent<HTMLInputElement>, field: {
        onChange: (value: number) => void
    }) => {
        const {value} = event.target;
        if (value.length === 0) {
            field.onChange(0);
            return;
        }
        if (/^\D$/.test(value[value.length - 1])) {
            const newValue = value.slice(0, -1);
            if (newValue.length === 0) {
                field.onChange(0);
            } else {
                field.onChange(Number(newValue));
            }
            return;
        }
        field.onChange(Number(value));
    };

    function onSubmit(data: z.infer<typeof FormSchema>) {
        console.log(data);
    }


    return <div className="*:data-[slot=card]:shadow-xs grid grid-cols-1 gap-4
            px-4 *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card
            dark:*:data-[slot=card]:bg-card lg:px-6 h-full">
        <Card className="@container/card h-full">
            <CardHeader className="relative gap-5">
                <CardTitle>
                    <div className="flex gap-5 items-center">
                        <h1 className="text-xl font-bold text-primary tracking-tight">Modifica Stanza</h1>
                        <CommentButton />
                    </div>
                </CardTitle>
                <CardContent className="px-0">
                    <Form { ...form }>
                        <form onSubmit={ form.handleSubmit(onSubmit) } className="">
                            <div className="grid grid-cols-12 gap-5">
                                <div className="row-start-1 col-span-12">
                                    <div className="grid grid-cols-12 gap-5">
                                        <FormField
                                            control={ form.control }
                                            name="stanza"
                                            render={ ({field}) => (<div className="col-span-4">
                                                <FormItem>
                                                    <FormLabel>Stanza</FormLabel>
                                                    <Select onValueChange={ field.onChange }
                                                            defaultValue={ field.value }>
                                                        <FormControl>
                                                            <SelectTrigger className="w-full">
                                                                <SelectValue
                                                                    placeholder="Seleziona una stanza" />
                                                            </SelectTrigger>
                                                        </FormControl>
                                                        <SelectContent>
                                                            <SelectItem value="m@example.com">m@example.com</SelectItem>
                                                            <SelectItem value="m@google.com">m@google.com</SelectItem>
                                                            <SelectItem value="m@support.com">m@support.com</SelectItem>
                                                        </SelectContent>
                                                    </Select>
                                                    <FormMessage />
                                                </FormItem>
                                            </div>) }
                                        />
                                        <FormField
                                            control={ form.control }
                                            name="destinazione_uso"
                                            render={ ({field}) => (<div className="col-span-4">
                                                <FormItem>
                                                    <FormLabel>Destinazione Uso</FormLabel>
                                                    <Select onValueChange={ field.onChange }
                                                            defaultValue={ field.value }>
                                                        <FormControl>
                                                            <SelectTrigger className="w-full">
                                                                <SelectValue
                                                                    placeholder="" />
                                                            </SelectTrigger>
                                                        </FormControl>
                                                        <SelectContent>
                                                            <SelectItem value="m@example.com">m@example.com</SelectItem>
                                                            <SelectItem value="m@google.com">m@google.com</SelectItem>
                                                            <SelectItem value="m@support.com">m@support.com</SelectItem>
                                                        </SelectContent>
                                                    </Select>
                                                    <FormMessage />
                                                </FormItem>
                                            </div>) }
                                        />
                                        <FormField
                                            control={ form.control }
                                            name="piano"
                                            render={ ({field}) => (<div className="col-span-4">
                                                <FormItem>
                                                    <FormLabel>Piano</FormLabel>
                                                    <Select onValueChange={ field.onChange }
                                                            defaultValue={ field.value }>
                                                        <FormControl>
                                                            <SelectTrigger className="w-full">
                                                                <SelectValue
                                                                    placeholder="" />
                                                            </SelectTrigger>
                                                        </FormControl>
                                                        <SelectContent>
                                                            <SelectItem value="m@example.com">m@example.com</SelectItem>
                                                            <SelectItem value="m@google.com">m@google.com</SelectItem>
                                                            <SelectItem value="m@support.com">m@support.com</SelectItem>
                                                        </SelectContent>
                                                    </Select>
                                                    <FormMessage />
                                                </FormItem>
                                            </div>) }
                                        />
                                    </div>
                                </div>
                                {/* Altezza e Spessore Muro */ }
                                <div className="row-start-2 col-span-12">
                                    <div className="grid grid-cols-12 gap-5">
                                        <FormField
                                            control={ form.control }
                                            name="altezza"
                                            render={ ({field}) => (<div className="col-span-6">
                                                <FormItem>
                                                    <FormLabel className="flex items-center">
                                                        <p>Altezza</p>
                                                        <HelpBadge message="Il valore va inserito in cm" />
                                                    </FormLabel>
                                                    <Input value={ field.value }
                                                           onChange={ e => handleInputNumericChange(e, field) }
                                                           defaultValue={ field.value } />
                                                    <FormMessage />
                                                </FormItem>
                                            </div>) }
                                        />
                                        <FormField
                                            control={ form.control }
                                            name="spessore_muro"
                                            render={ ({field}) => (<div className="col-span-6">
                                                <FormItem>
                                                    <FormLabel className="flex items-center">
                                                        <p>Spessore Muro</p>
                                                        <HelpBadge message="Il valore va inserito in cm" />
                                                    </FormLabel>
                                                    <Input value={ field.value }
                                                           onChange={ e => handleInputNumericChange(e, field) } />
                                                    <FormMessage />
                                                </FormItem>
                                            </div>) }
                                        />
                                    </div>
                                </div>
                                {/* Riscaldamento e altro */ }
                                <div className="row-start-3 col-span-12">
                                    <div className="grid grid-cols-12 gap-5">
                                        <FormField
                                            control={ form.control }
                                            name="riscaldamento"
                                            render={ ({field}) => (<div className="col-span-6">
                                                <FormItem>
                                                    <FormLabel>Riscaldamento</FormLabel>
                                                    <Select onValueChange={ field.onChange }
                                                            defaultValue={ field.value }>
                                                        <FormControl>
                                                            <SelectTrigger className="w-full">
                                                                <SelectValue
                                                                    placeholder="Seleziona una stanza" />
                                                            </SelectTrigger>
                                                        </FormControl>
                                                        <SelectContent>
                                                            <SelectItem value="m@example.com">m@example.com</SelectItem>
                                                            <SelectItem value="m@google.com">m@google.com</SelectItem>
                                                            <SelectItem value="m@support.com">m@support.com</SelectItem>
                                                        </SelectContent>
                                                    </Select>
                                                    <FormMessage />
                                                </FormItem>
                                            </div>) }
                                        />
                                        <FormField
                                            control={ form.control }
                                            name="riscaldamento"
                                            render={ ({field}) => (<div className="col-span-6">
                                                <FormItem>
                                                    <FormLabel>Specifica altro riscaldamento</FormLabel>
                                                    <Select onValueChange={ field.onChange }
                                                            defaultValue={ field.value }>
                                                        <FormControl>
                                                            <SelectTrigger className="w-full">
                                                                <SelectValue
                                                                    placeholder="" />
                                                            </SelectTrigger>
                                                        </FormControl>
                                                        <SelectContent>
                                                            <SelectItem value="m@example.com">m@example.com</SelectItem>
                                                            <SelectItem value="m@google.com">m@google.com</SelectItem>
                                                            <SelectItem value="m@support.com">m@support.com</SelectItem>
                                                        </SelectContent>
                                                    </Select>
                                                    <FormMessage />
                                                </FormItem>
                                            </div>) }
                                        />
                                    </div>
                                </div>
                                {/* Raffrescamento e altro */ }
                                <div className="row-start-4 col-span-12">
                                    <div className="grid grid-cols-12 gap-5">
                                        <FormField
                                            control={ form.control }
                                            name="raffrescamento"
                                            render={ ({field}) => (<div className="col-span-6">
                                                <FormItem>
                                                    <FormLabel>Raffrescamento</FormLabel>
                                                    <Select onValueChange={ field.onChange }
                                                            defaultValue={ field.value }>
                                                        <FormControl>
                                                            <SelectTrigger className="w-full">
                                                                <SelectValue
                                                                    placeholder="Seleziona una stanza" />
                                                            </SelectTrigger>
                                                        </FormControl>
                                                        <SelectContent>
                                                            <SelectItem value="m@example.com">m@example.com</SelectItem>
                                                            <SelectItem value="m@google.com">m@google.com</SelectItem>
                                                            <SelectItem value="m@support.com">m@support.com</SelectItem>
                                                        </SelectContent>
                                                    </Select>
                                                    <FormMessage />
                                                </FormItem>
                                            </div>) }
                                        />
                                        <FormField
                                            control={ form.control }
                                            name="raffrescamento"
                                            render={ ({field}) => (<div className="col-span-6">
                                                <FormItem>
                                                    <FormLabel>Specifica altro raffrescamento</FormLabel>
                                                    <Select onValueChange={ field.onChange }
                                                            defaultValue={ field.value }>
                                                        <FormControl>
                                                            <SelectTrigger className="w-full">
                                                                <SelectValue
                                                                    placeholder="" />
                                                            </SelectTrigger>
                                                        </FormControl>
                                                        <SelectContent>
                                                            <SelectItem value="m@example.com">m@example.com</SelectItem>
                                                            <SelectItem value="m@google.com">m@google.com</SelectItem>
                                                            <SelectItem value="m@support.com">m@support.com</SelectItem>
                                                        </SelectContent>
                                                    </Select>
                                                    <FormMessage />
                                                </FormItem>
                                            </div>) }
                                        />
                                    </div>
                                </div>
                                {/* Illuminazione e altro */ }
                                <div className="row-start-5 col-span-12">
                                    <div className="grid grid-cols-12 gap-5">
                                        <FormField
                                            control={ form.control }
                                            name="illuminazione"
                                            render={ ({field}) => (<div className="col-span-6">
                                                <FormItem>
                                                    <FormLabel>Illuminazione</FormLabel>
                                                    <Select onValueChange={ field.onChange }
                                                            defaultValue={ field.value }>
                                                        <FormControl>
                                                            <SelectTrigger className="w-full">
                                                                <SelectValue
                                                                    placeholder="Seleziona una stanza" />
                                                            </SelectTrigger>
                                                        </FormControl>
                                                        <SelectContent>
                                                            <SelectItem value="m@example.com">m@example.com</SelectItem>
                                                            <SelectItem value="m@google.com">m@google.com</SelectItem>
                                                            <SelectItem value="m@support.com">m@support.com</SelectItem>
                                                        </SelectContent>
                                                    </Select>
                                                    <FormMessage />
                                                </FormItem>
                                            </div>) }
                                        />
                                        <FormField
                                            control={ form.control }
                                            name="illuminazione"
                                            render={ ({field}) => (<div className="col-span-6">
                                                <FormItem>
                                                    <FormLabel>Specifica altro illuminazione</FormLabel>
                                                    <Select onValueChange={ field.onChange }
                                                            defaultValue={ field.value }>
                                                        <FormControl>
                                                            <SelectTrigger className="w-full">
                                                                <SelectValue
                                                                    placeholder="" />
                                                            </SelectTrigger>
                                                        </FormControl>
                                                        <SelectContent>
                                                            <SelectItem value="m@example.com">m@example.com</SelectItem>
                                                            <SelectItem value="m@google.com">m@google.com</SelectItem>
                                                            <SelectItem value="m@support.com">m@support.com</SelectItem>
                                                        </SelectContent>
                                                    </Select>
                                                    <FormMessage />
                                                </FormItem>
                                            </div>) }
                                        />
                                    </div>
                                </div>
                                {/* Infissi */ }
                                <div className="row-start-6 col-span-12">
                                    <FormField
                                        control={ form.control }
                                        name="infissi"
                                        render={ ({field}) => (<FormItem>
                                            <FormLabel>Infissi</FormLabel>
                                            <FormControl>
                                                <DynamicSelect onChange={ field.onChange } value={ field.value } />
                                            </FormControl>
                                            <FormMessage />
                                        </FormItem>) }
                                    />
                                </div>
                            </div>
                            <div className="flex justify-end pt-4">
                                <Button type="submit" className="text-white">
                                    <Pencil /> <span>Modifica Stanza</span>
                                </Button>
                            </div>
                        </form>
                    </Form>
                </CardContent>
            </CardHeader>
        </Card>
    </div>;
};

export default CardFormStanza;
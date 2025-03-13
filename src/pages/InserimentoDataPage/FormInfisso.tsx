import Label          from "../../components/Label.tsx";
import * as React     from "react";
import { useState }   from "react";
import Input          from "../../components/Input.tsx";
import Select         from "../../components/Select.tsx";
import { useTypes }   from "../../context/TypesProvider.tsx";
import { useInfissi } from "../../context/InfissiProvider.tsx";
import { IInfisso }   from "../../models/models.tsx";

const nextAlphabeticalID = (prevID: string | null) => {
    if (!prevID || prevID === "") return "A";
    let result = "";
    let carry  = true;
    for (let i = prevID.length - 1; i >= 0; i--) {
        const char = prevID[i];
        if (carry) {
            if (char === "Z") {
                result = "A" + result;
            } else {
                result = String.fromCharCode(char.charCodeAt(0) + 1) + result;
                carry  = false;
            }
        } else {
            result = char + result;
        }
    }
    return carry ? "A" + result : result;
};


const FormInfisso = () => {
    const [ formData, setFormData ] = useState<IInfisso>({
        altezza  : 0,
        larghezza: 0,
        materiale: "",
        vetro    : ""
    });
    const {
              materialiInfissiType,
              vetroInfissiType
          }                         = useTypes();
    const infissi                   = useInfissi();

    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        if (formData.altezza === 0 && formData.larghezza === 0 && formData.materiale === "" && formData.vetro === "") {
            return;
        }
        const lastInfisso = infissi.data.at(-1);
        console.log("LastInfisso: ");
        console.log(lastInfisso);
        let lastInfissoId = "";
        if (lastInfisso) {
            if (lastInfisso.id) {
                lastInfissoId = lastInfisso.id;
            } else {
                throw new Error("Infisso doesn't have id");
            }

        }

        const newInfisso: IInfisso = {
            ...formData,
            id: nextAlphabeticalID(lastInfissoId)
        };
        console.log(newInfisso);
        infissi.updateInfissi(newInfisso);
    };

    return (<form onSubmit={ handleSubmit } className="space-y-4">
        <h2 className="text-2xl font-bold text-gray-800 mb-6 border-b pb-3">
            Inserimento nuovo infisso
        </h2>

        <div className="grid grid-cols-12 gap-8">
            {/* Altezza e Larghezza */ }
            <div className="row-start-1 col-span-12">
                <div className="grid grid-cols-12 items-center gap-5">
                    <Label htmlFor="altezza" className="col-span-2"> Altezza </Label>
                    <Input name="altezza"
                           value={ formData.altezza }
                           onChange={ (e) => {
                               setFormData((prev) => ({
                                   ...prev,
                                   altezza: Number(e.target.value)
                               }));
                           } }
                           className="col-span-4"
                    />
                    <Label htmlFor="larghezza" className="col-span-2">Larghezza</Label>
                    <Input name="larghezza"
                           value={ formData.larghezza }
                           onChange={ (e) => {
                               setFormData((prev) => ({
                                   ...prev,
                                   larghezza: Number(e.target.value)
                               }));
                           } }
                           className="col-span-4"
                    />
                </div>
            </div>
            {/* Materiale e Vetro */ }
            <div className="row-start-2 col-span-12">
                <div className="grid grid-cols-12 items-center gap-5">
                    <Label htmlFor="materiale" className="col-span-2">Materiale</Label>
                    <Select name="materiale"
                            value={ formData.materiale }
                            onChange={ (e) => {
                                setFormData((prev) => ({
                                    ...prev,
                                    materiale: e.target.value
                                }));
                            } }
                            className="col-span-4"
                    >
                        { materialiInfissiType.map((item, index) => (<option key={ index }>{ item }</option>)) }
                    </Select>
                    <Label htmlFor="vetro" className="col-span-2">Vetro</Label>
                    <Select name="vetro"
                            value={ formData.vetro }
                            onChange={ (e) => {
                                setFormData((prev) => ({
                                    ...prev,
                                    vetro: e.target.value
                                }));
                            } }
                            className="col-span-4"
                    >
                        { vetroInfissiType.map((item, index) => (<option key={ index }>{ item }</option>)) }
                    </Select>
                </div>
            </div>
        </div>
        <div className="flex justify-end pt-8">
            <button
                type="submit"
                className="w-80 bg-blue-600 text-white py-2 rounded-md
                             hover:bg-blue-700 transition-colors duration-300
                             focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
            >
                Inserisci
            </button>
        </div>
    </form>);
};


export default FormInfisso;
import * as React              from "react";
import { useState }            from "react";
import Label                   from "../../components/Label";
import Input                   from "../../components/Input.tsx";
import Select, { SingleValue } from "react-select";
import { useTypes }            from "../../context/TypesProvider.tsx";
import CommentsButton          from "../../components/CommentsButton.tsx";
import DynamicSelectsInfissi   from "../../components/DynamicSelectsInfissi.tsx";

interface RoomSpecifications {
    stanza: string,
    altezza: number,
    spessoreMuro: number,
    riscaldamento: string,
    altroRiscaldamento?: string,
    raffreddamento: string,
    altroRaffreddamento?: string,
    illuminazione: string,
    altroIlluminazione?: string,
}


const FormStanza = () => {
    const [ formData, setFormData ]           = useState<RoomSpecifications>({
        stanza        : "",
        altezza       : 0,
        spessoreMuro  : 0,
        riscaldamento : "",
        raffreddamento: "",
        illuminazione : ""
    });
    const [ infissiValues, setInfissiValues ] = useState<string[]>([ "" ]);

    const [ altro, setAltro ]        = useState({
        riscaldamento : true,
        raffreddamento: true,
        illuminazione : true
    });
    const {
              illuminazioneType,
              climatizzazioneType
          }                          = useTypes();
    const illuminazioneTypeOptions   = [
        ...illuminazioneType.map((item) => ({
            value: item,
            label: item
        })), {
            value: "altro",
            label: "Altro"
        }
    ];
    const climatizzazioneTypeOptions = [
        ...climatizzazioneType.map((item) => ({
            value: item,
            label: item
        })), {
            value: "altro",
            label: "Altro"
        }
    ];

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const {
                  name,
                  value
              } = e.target;
        setFormData(prevState => ({
            ...prevState,
            [name]: value
        }));
    };

    const handleSelect = (newValue: SingleValue<{ label: string, value: string }>, name: string) => {
        if (newValue?.value) {
            setAltro((prev) => ({
                ...prev,
                [name]: newValue.value !== "altro"
            }));
            setFormData((prev) => ({
                ...prev,
                [name]: newValue.value
            }));
        }
    };


    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        console.log("Form submitted:", formData);
        console.log("Infissi:", infissiValues);
    };

    return <form onSubmit={ handleSubmit } className="space-y-4">
        <div className="border-b flex justify-start items-center gap-2.5 mb-6 pb-3">
            <h2 className="text-2xl font-bold text-gray-800">
                Dati Stanza
            </h2>
            {/* todo: add function to save comment */ }
            <CommentsButton saveComment={ () => {
            } } />
        </div>

        <div className="grid grid-cols-12 gap-5">
            {/* Stanza */ }
            <div className="row-start-1 col-span-12">
                <div className="grid grid-cols-12 items-center gap-5">
                    <Label htmlFor="stanza" className="col-span-2"> Stanza/e </Label>
                    <Input name="stanza"
                           value={ formData.stanza }
                           onChange={ handleChange }
                           placeholder="Inserisci il numero della stanza"
                           className="col-span-4"
                    />
                    <Label htmlFor="stanza" className="col-span-2"> Destinazione D'uso </Label>
                    <Input name="stanza"
                           value={ formData.stanza }
                           onChange={ handleChange }
                           placeholder="Inserisci la destinazione d'uso"
                           className="col-span-4"
                    />
                </div>
            </div>
            {/* Altezza */ }
            <div className="row-start-2 col-span-12">
                <div className="grid grid-cols-12 items-center gap-5">
                    <Label htmlFor="altezza" className="col-span-2"> Altezza (cm) </Label>
                    <Input name="altezza"
                           value={ formData.altezza }
                           onChange={ handleChange }
                           className="col-span-4"
                    />
                    <Label htmlFor="spessoreMuro" className="col-span-2">Spessore Muro (cm)</Label>
                    <Input name="spessoreMuro"
                           value={ formData.spessoreMuro }
                           onChange={ handleChange }
                           className="col-span-4"
                    />
                </div>
            </div>
            {/* Infissi */ }
            <div className="row-start-3 col-span-12">
                <div className="grid grid-cols-12 items-baseline gap-5">
                    <Label htmlFor="infisso" className="col-span-2">Infissi</Label>
                    <div className="col-span-10">
                        <DynamicSelectsInfissi infissiValues={ infissiValues } setInfissiValues={ setInfissiValues } />
                    </div>
                </div>
            </div>
            {/* Riscaldamento */ }
            <div className="row-start-4 col-span-12">
                <div className="grid grid-cols-12 items-center gap-5">
                    <Label htmlFor="riscaldamento" className="col-span-2">Riscaldamento</Label>
                    <Select name="riscaldamento"
                            onChange={ (newValue: SingleValue<{ value: string, label: string }>) => {
                                handleSelect(newValue, "riscaldamento");
                            } }
                            className="col-span-4 rounded-md shadow-sm"
                            options={ climatizzazioneTypeOptions }
                    />
                    <Label htmlFor="altroRiscaldamento">Altro</Label>
                    <Input name="altroRiscaldamento" value={ formData.altroRiscaldamento ?? "" }
                           onChange={ handleChange }
                           className="col-span-5 disabled:bg-gray-200" disabled={ altro["riscaldamento"] } />
                </div>
            </div>
            {/* Raffreddamento */ }
            <div className="row-start-5 col-span-12">
                <div className="grid grid-cols-12 items-center gap-5">
                    <Label htmlFor="raffreddamento" className="col-span-2">Raffreddamento</Label>
                    <Select name="raffreddamento"
                            onChange={ (newValue: SingleValue<{ value: string, label: string }>) => {
                                handleSelect(newValue, "raffreddamento");
                            } }
                            className="col-span-4 rounded-md shadow-sm"
                            options={ climatizzazioneTypeOptions }
                    />
                    <Label htmlFor="altroRaffreddamento">Altro</Label>
                    <Input name="altroRaffreddamento" value={ formData.altroRaffreddamento ?? "" }
                           onChange={ handleChange }
                           className="col-span-5 disabled:bg-gray-200" disabled={ altro["raffreddamento"] } />
                </div>
            </div>
            {/* Illuminazione */ }
            <div className="row-start-6 col-span-12">
                <div className="grid grid-cols-12 items-center gap-5">
                    <Label htmlFor="illuminazione" className="col-span-2">Illuminazione</Label>
                    <Select name="illuminazione"
                            onChange={ (newValue: SingleValue<{ value: string, label: string }>) => {
                                handleSelect(newValue, "illuminazione");
                            } }
                            className="col-span-4 rounded-md shadow-sm"
                            options={ illuminazioneTypeOptions }
                    />
                    <Label htmlFor="altroIlluminazione">Altro</Label>
                    <Input name="altroIlluminazione" value={ formData.altroIlluminazione ?? "" }
                           onChange={ handleChange }
                           className="col-span-5 disabled:bg-gray-200"
                           disabled={ altro["illuminazione"] }
                    />
                </div>
            </div>
        </div>
        <div className="flex justify-end pt-4">
            <button
                type="submit"
                className="w-80 bg-blue-600 text-white py-2 rounded-md
                             hover:bg-blue-700 transition-colors duration-300
                             focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
            >
                Inserisci
            </button>
        </div>
    </form>;
};

export default FormStanza;
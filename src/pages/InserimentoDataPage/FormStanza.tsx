import * as React     from "react";
import { useState }   from "react";
import Label          from "../../components/Label";
import Input          from "../../components/Input.tsx";
import Select         from "../../components/Select.tsx";
import { useTypes }   from "../../context/TypesProvider.tsx";
import { useInfissi } from "../../context/InfissiProvider.tsx";

interface RoomSpecifications {
    stanza: string,
    altezza: number,
    spessoreMuro: number,
    infisso: string,
    riscaldamento: string,
    altroRiscaldamento?: string,
    raffreddamento: string,
    altroRaffreddamento?: string,
    illuminazione: string,
    altroIlluminazione?: string,
}


const FormStanza = () => {
    const [ formData, setFormData ] = useState<RoomSpecifications>({
        stanza        : "",
        altezza       : 0,
        spessoreMuro  : 0,
        infisso       : "",
        riscaldamento : "",
        raffreddamento: "",
        illuminazione : ""
    });
    const [ altro, setAltro ]       = useState({
        riscaldamento : true,
        raffreddamento: true,
        illuminazione : true
    });
    const {
              illuminazioneType,
              climatizzazioneType
          }                         = useTypes();
    const infissi                   = useInfissi();


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

    const handleSelect = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const {
                  name,
                  value
              } = e.target;
        console.log(name, value);
        setAltro((prev) => ({
            ...prev,
            [name]: value !== "altro"
        }));
        setFormData((prev) => ({
            ...prev,
            [name]: value
        }));

    };


    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        console.log("Form submitted:", formData);
    };

    return <form onSubmit={ handleSubmit } className="space-y-4">
        <h2 className="text-2xl font-bold text-gray-800 mb-6 border-b pb-3">
            Dati Stanza
        </h2>

        <div className="grid grid-cols-12 gap-5">
            {/* Stanza */ }
            <div className="row-start-1 col-span-12">
                <div className="grid grid-cols-12 items-center gap-5">
                    <Label htmlFor="stanza" className="col-span-2"> Stanza/e </Label>
                    <Input name="stanza"
                           value={ formData.stanza }
                           onChange={ handleChange }
                           placeholder="Inserisci il tuo nome"
                           className="col-span-10"
                    />
                </div>
            </div>
            {/* Altezza */ }
            <div className="row-start-2 col-span-12">
                <div className="grid grid-cols-12 items-center gap-5">
                    <Label htmlFor="altezza" className="col-span-2"> Altezza </Label>
                    <Input name="altezza"
                           value={ formData.altezza }
                           onChange={ handleChange }
                           className="col-span-4"
                    />
                    <Label htmlFor="spessoreMuro" className="col-span-2">Spessore Muro</Label>
                    <Input name="spessoreMuro"
                           value={ formData.spessoreMuro }
                           onChange={ handleChange }
                           className="col-span-4"
                    />
                </div>
            </div>
            {/* Infissi */ }
            <div className="row-start-3 col-span-12">
                <div className="grid grid-cols-12 items-center gap-5">
                    <Label htmlFor="infisso" className="col-span-2">Infissi</Label>
                    <Select name="infisso"
                            value={ formData.infisso }
                            onChange={ handleSelect }
                            className="col-span-10"
                    >
                        { infissi.data.map((item) => (<option key={ item.id } value={ item.id }>{ item.id }</option>)) }
                    </Select>
                </div>
            </div>
            {/* Riscaldamento */ }
            <div className="row-start-4 col-span-12">
                <div className="grid grid-cols-12 items-center gap-5">
                    <Label htmlFor="riscaldamento" className="col-span-2">Riscaldamento</Label>
                    <Select name="riscaldamento"
                            value={ formData.riscaldamento }
                            onChange={ handleSelect }
                            className="col-span-4"
                            optionAltro={ true }
                    >
                        { climatizzazioneType.map((item, index) => (
                            <option key={ index } value={ item }>{ item }</option>)) }
                    </Select>
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
                            value={ formData.raffreddamento }
                            onChange={ handleSelect }
                            className="col-span-4"
                            optionAltro={ true }
                    >
                        { climatizzazioneType.map((item, index) => (
                            <option key={ index } value={ item }>{ item }</option>)) }
                    </Select>
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
                            value={ formData.illuminazione }
                            onChange={ handleSelect }
                            className="col-span-4"
                            optionAltro={ true }
                    >
                        { illuminazioneType.map((item, index) => (
                            <option key={ index } value={ item }>{ item }</option>)) }
                    </Select>
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
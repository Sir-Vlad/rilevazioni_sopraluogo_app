import Label                                 from "../../components/Label.tsx";
import * as React                            from "react";
import { ChangeEvent, useState }             from "react";
import Input                                 from "../../components/Input.tsx";
import Select, { SingleValue }               from "react-select";
import { useDatabase, useInfissi, useTypes } from "../../context/UseProvider.tsx";
import { IInfisso }                          from "../../models/models.tsx";
import { toast }                             from "react-toastify";
import CommentsButton                        from "../../components/CommentsButton.tsx";


const nextAlphabeticalID = (prevID: string | null) => {
    if (!prevID || prevID === "") return "A";
    let result = "";
    let carry = true;
    for (let i = prevID.length - 1; i >= 0; i--) {
        const char = prevID[i];
        if (carry) {
            if (char === "Z") {
                result = "A" + result;
            } else {
                result = String.fromCharCode(char.charCodeAt(0) + 1) + result;
                carry = false;
            }
        } else {
            result = char + result;
        }
    }
    return carry ? "A" + result : result;
};


const FormInfisso = () => {
    const [ formData, setFormData ] = useState<IInfisso>({
        tipo     : "Finestra",
        altezza  : 0,
        larghezza: 0,
        materiale: "",
        vetro    : ""
    });
    const {
              materialiInfissiType,
              vetroInfissiType
          } = useTypes();
    const infissi = useInfissi();
    const {error} = useDatabase();

    const handleTipoChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const {value} = e.target;
        setFormData((prev) => ({
            ...prev,
            tipo: value
        }));
    };

    const handleInputNumericChange = (e: ChangeEvent<HTMLInputElement>) => {
        const {
                  name,
                  value
              } = e.target;
        setFormData((prev) => {
            let newValue = 0;
            if (value.length > 0) {
                newValue = /^\d*$/.test(value[value.length - 1]) ? Number(value) : Number(prev[name as keyof IInfisso]);
            }
            return {
                ...prev,
                [name]: newValue
            };
        });
    };

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        if (error === "Database non settato") {
            toast.warning("File non selezionato");
            return;
        }

        if (formData.altezza === 0 && formData.larghezza === 0 && formData.materiale === "" && formData.vetro === "") {
            return;
        }
        const lastInfisso = infissi.data.at(-1);
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
        try {
            await infissi.insertInfisso(newInfisso);
            toast.success("Infisso inserito con successo");
        } catch (e) {
            toast.error("Errore durante l'inserimento del nuovo infisso");
            console.error(e);
        }
        setFormData({
            tipo     : "Finestra",
            altezza  : 0,
            larghezza: 0,
            materiale: "",
            vetro    : ""
        });
    };

    return (<form onSubmit={ handleSubmit } className="space-y-4">
        <div className="border-b flex justify-start items-center gap-2.5 mb-6 pb-3">
            <span className="text-2xl font-bold text-gray-800">
                Inserimento nuovo infisso
            </span>
            {/* todo: add function to save comment */ }
            <CommentsButton saveComment={ () => {
            } } />
        </div>

        <div className="grid grid-cols-12 gap-8">
            {/* Tipo */ }
            <div className="row-start-1 col-span-12">
                <div className="grid grid-cols-12 items-center">
                    <Label htmlFor="tipo" className="col-span-2">Tipo</Label>
                    <div id="tipo" className="col-span-4 flex items-center justify-start gap-4">
                        <div className="col-span-2 flex items-center gap-2 px-3 py-2">
                            <input type="radio" id="chk_finestra"
                                   name="check_tipo" value="Finestra"
                                   className="h-4 w-4 accent-blue-500"
                                   checked={ formData.tipo === "Finestra" }
                                   onChange={ handleTipoChange }
                            />
                            <label htmlFor="chk_finestra">Finestra</label>
                        </div>
                        <div className="col-span-2 flex items-center gap-2 px-3 py-2">
                            <input type="radio" id="chk_finestra"
                                   name="check_tipo" value="Porta"
                                   className="h-4 w-4 accent-blue-500"
                                   checked={ formData.tipo === "Porta" }
                                   onChange={ handleTipoChange }
                            />
                            <label htmlFor="chk_porta">Porta</label>
                        </div>
                    </div>
                </div>
            </div>
            {/* Altezza e Larghezza */ }
            <div className="row-start-2 col-span-12">
                <div className="grid grid-cols-12 items-center gap-5">
                    <Label htmlFor="altezza" className="col-span-2">Altezza (cm)</Label>
                    <Input name="altezza"
                           value={ formData.altezza }
                           onChange={ handleInputNumericChange }
                           className="col-span-4 decoration-none"
                    />
                    <Label htmlFor="larghezza" className="col-span-2">Larghezza (cm)</Label>
                    <Input name="larghezza"
                           value={ formData.larghezza }
                           onChange={ handleInputNumericChange }
                           className="col-span-4"
                    />
                </div>
            </div>
            {/* Materiale e Vetro */ }
            <div className="row-start-3 col-span-12">
                <div className="grid grid-cols-12 items-center gap-5">
                    <Label htmlFor="materiale" className="col-span-2">Materiale</Label>
                    <Select name="materiale"
                            onChange={ (newValue: SingleValue<{ label: string, value: string }>) => {
                                if (newValue?.value) {
                                    setFormData((prev) => ({
                                        ...prev,
                                        materiale: newValue.value
                                    }));
                                }
                            } }
                            className="col-span-4"
                            options={ materialiInfissiType.map((item) => ({
                                label: item,
                                value: item
                            })) }
                    />
                    <Label htmlFor="vetro" className="col-span-2">Vetro</Label>
                    <Select name="vetro"
                            onChange={ (newValue: SingleValue<{ label: string, value: string }>) => {
                                if (newValue?.value) {
                                    setFormData((prev) => ({
                                        ...prev,
                                        vetro: newValue.value
                                    }));
                                }
                            } }
                            className="col-span-4"
                            options={ vetroInfissiType.map((item) => ({
                                label: item,
                                value: item
                            })) }
                    />
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
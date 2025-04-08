import * as React                           from "react";
import { ChangeEvent, useEffect, useState } from "react";
import Label                                from "../../components/Label";
import Input                                from "../../components/Input.tsx";
import Select, { SingleValue }              from "react-select";
import { useDatabase, useStanze, useTypes } from "../../context/UseProvider.tsx";
import CommentsButton                       from "../../components/CommentsButton.tsx";
import DynamicSelectsInfissi                from "../../components/DynamicSelectsInfissi.tsx";
import { toast }                            from "react-toastify";
import { capitalize }                       from "../../helpers/helpers.tsx";

interface RoomSpecifications {
    stanza: string,
    destinazioneUso: string,
    piano?: string,
    cappotto: boolean,
    altezza: number,
    spessoreMuro: number,
    riscaldamento: string,
    altroRiscaldamento?: string,
    raffrescamento: string,
    altroRaffrescamento?: string,
    illuminazione: string,
    altroIlluminazione?: string,
}

type SelectOption = {
    value: string, label: string
};

type AltroType = {
    riscaldamento: boolean, raffrescamento: boolean, illuminazione: boolean
};

const FormStanza = () => {
    const [ formData, setFormData ] = useState<RoomSpecifications>({
        stanza         : "",
        destinazioneUso: "",
        cappotto       : false,
        altezza        : 0,
        spessoreMuro   : 0,
        riscaldamento  : "",
        raffrescamento : "",
        illuminazione  : ""
    });
    const [ infissiValues, setInfissiValues ] = useState<string[]>([ "" ]);

    const [ altro, setAltro ] = useState<AltroType>({
        riscaldamento : true,
        raffrescamento: true,
        illuminazione : true
    });
    const {
              illuminazioneType,
              climatizzazioneType
          } = useTypes();
    const stanze = useStanze();
    const {error} = useDatabase();

    const illuminazioneTypeOptions = [
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

    const stanzeOptions = [
        ...[ ...new Set(stanze.data.map((item) => item.stanza)) ]
            .sort((a, b) => {
                if (a.startsWith("_") && !b.startsWith("_")) return -1;
                if (!a.startsWith("_") && b.startsWith("_")) return 1;
                const aNum = Number(a);
                const bNum = Number(b);
                const aIsNum = !isNaN(aNum);
                const bIsNum = !isNaN(bNum);
                if (aIsNum && bIsNum) return aNum - bNum;
                if (aIsNum) return -1;
                if (bIsNum) return 1;
                return a.localeCompare(b);
            })
            .map((item) => ({
                value: item,
                label: item
            }))
    ];

    const [ destinazioneUsoOptions, setDestinazioneUsoOptions ] = useState<SelectOption[]>([
        {
            label: "",
            value: ""
        }
    ]);
    const [ pianoOptions, setPianoOptions ] = useState<SelectOption[]>([
        {
            label: "",
            value: ""
        }
    ]);

    useEffect(() => {
        if (destinazioneUsoOptions.length === 1) {
            setFormData((prev) => ({
                ...prev,
                destinazioneUso: destinazioneUsoOptions[0].value
            }));
        }
        if (pianoOptions.length === 1) {
            setFormData((prev) => ({
                ...prev,
                piano: pianoOptions[0].value
            }));
        }
    }, [ destinazioneUsoOptions, pianoOptions ]);

    const currentValueDestinazioneUso = () => {
        if (destinazioneUsoOptions.length <= 1) {
            return destinazioneUsoOptions[0];
        }
        if (!formData.destinazioneUso) {
            return null;
        }
        return {
            value: formData.destinazioneUso,
            label: formData.destinazioneUso
        };
    };

    const handleStanzaChange = (newValue: SingleValue<{ label: string, value: string }>) => {
        if (newValue?.value) {
            setFormData((prev) => ({
                ...prev,
                stanza         : newValue.value,
                destinazioneUso: ""
            }));

            const destinazioneUsoStanze = stanze.data.filter((item) => item.stanza === newValue.value)
                                                .map((item) => {
                                                    return item.destinazione_uso;
                                                });
            setDestinazioneUsoOptions([ ...new Set(destinazioneUsoStanze) ].map((value): SelectOption => ({
                label: value,
                value: value
            })));
            const pianoStanze = stanze.data.filter((item) => item.stanza === newValue.value)
                                      .map((item) => {
                                          return item.piano;
                                      });
            setPianoOptions([ ...new Set(pianoStanze) ].map((value): SelectOption => ({
                label: value,
                value: value
            })));
        }
    };

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

    const handleInputNumericChange = (e: ChangeEvent<HTMLInputElement>) => {
        const {
                  name,
                  value
              } = e.target;
        setFormData((prev) => {
            let newValue = 0;
            if (value.length > 0) {
                newValue = /^\d*$/.test(value[value.length - 1]) ? Number(value) : Number(prev[name as keyof RoomSpecifications]);
            }
            return {
                ...prev,
                [name]: newValue
            };
        });
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

    /*****************************************************************************************************************
     ************************************  INSERIMENTO STANZE NEL DATABASE  ******************************************
     *****************************************************************************************************************/

    const IGNORED_FIELD_KEYS = [ "altroRiscaldamento", "altroRaffrescamento", "altroIlluminazione", "cappotto" ];
    const ALTRO_FIELDS = [ "riscaldamento", "raffrescamento", "illuminazione" ];
    const fieldEmpty: string[] = [];

    /**
     * Determines if a given field should be skipped based on its key.
     *
     * The function checks whether the provided key exists in the predefined list of ignored field keys.
     *
     * @param {string} key - The key of the field to evaluate.
     * @returns {boolean} - Returns true if the field key is present in the ignored field keys, otherwise false.
     */
    const shouldSkipField = (key: string): boolean => IGNORED_FIELD_KEYS.includes(key);

    /**
     * Validates if the provided field key and value require an additional "altro" field to be filled out.
     *
     * This function checks if the `value` equals "altro" and whether the corresponding `altro` field
     * from the `AltroType` object is empty. If the specific `altro` field is empty, it determines the key
     * for the "altro" field, checks its presence in the `formData` object, and adds it to the `fieldEmpty`
     * array if missing.
     *
     * @param {string} key - The key representing the field being validated.
     * @param {string | number} value - The value of the field being validated.
     * @returns {boolean} Returns a boolean indicating if the additional "altro" field is filled (true)
     * or missing (false).
     */
    const validateAltroField = (key: string, value: string | number): boolean => {
        if (value === "altro" && !altro[key as keyof AltroType]) {
            if (!altro[key as keyof AltroType]) {
                const altroKey: string = "altro" + capitalize(key);
                const res = !!formData[altroKey as keyof RoomSpecifications];
                if (!res) {
                    fieldEmpty.push(altroKey);
                }
                return res;
            }
        }
        return true;
    };

    /**
     * Validates a specific field based on the provided key and value.
     *
     * The function determines whether the field should be skipped,
     * validated against a specific set of rules for certain field keys,
     * or checked for general validity. If the field is invalid and empty,
     * its key is added to a list of empty fields.
     *
     * @param {string} key - The identifier of the field to validate.
     * @param {string | number} value - The value of the field to validate.
     * @returns {boolean} - Returns true if the field is valid or should be skipped.
     *                      Returns false if the field is invalid.
     */
    const validateField = (key: string, value: string | number): boolean => {
        if (shouldSkipField(key)) return true;
        if (ALTRO_FIELDS.includes(key)) return validateAltroField(key, value);
        const isValid = !!value;
        if (!isValid) {
            fieldEmpty.push(key);
        }
        return isValid;
    };

    /**
     * Function to display a toast error for each field that is considered empty in the form.
     * It iterates through the `fieldEmpty` array, checks the field names, adjusts them for readability,
     * and triggers a warning toast message indicating that the field is not filled out.
     *
     * Adjustments to field names:
     * - If the field name is "destinazioneUso", it is displayed as "destinazione d'uso".
     * - If the field name is "spessoreMuro", it is displayed as "spessore muro".
     * - If the field name matches the pattern /^altro.*$/, it is displayed as "altro di <suffix>",
     *   where <suffix> is the remaining part of the field name after "altro" converted to lowercase.
     *
     * Emits a warning toast with the message: "Campo <field> non compilato" for each unfilled field.
     */
    const printToastError = () => {
        fieldEmpty.forEach((field) => {
            if (field === "destinazioneUso") {
                field = "destinazione d'uso";
            } else if (field === "spessoreMuro") {
                field = "spessore muro";
            } else if (/^altro.*$/.test(field)) {
                field = "altro di " + field.split("altro")[1].toLowerCase();
            }
            toast.warning("Campo " + field + " non compilato");
        });
        // clear array
        fieldEmpty.length = 0;
    };

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        if (error === "Database non settato") {
            toast.warning("File non selezionato");
            return;
        }

        let isFormCorrect = Object.entries(formData)
                                  .every(([ key, value ]: [ string, string | number ]) => validateField(key, value));
        if (infissiValues[0] === "") {
            isFormCorrect = false;
            fieldEmpty.push("infissi");
        }
        if (!isFormCorrect) {
            printToastError();
            return;
        }
        const stanza = stanze.data.find((item) => {
            return item.stanza === formData.stanza && item.destinazione_uso === formData.destinazioneUso && item.piano === formData.piano;
        });
        if (stanza === undefined) {
            toast.error("Stanza non trovata");
            return;
        }
        stanza.cappotto = formData.cappotto;
        stanza.altezza = formData.altezza;
        stanza.spessore_muro = formData.spessoreMuro;
        stanza.riscaldamento = formData.riscaldamento === "altro" ? formData.altroRiscaldamento : formData.riscaldamento;
        stanza.raffrescamento = formData.raffrescamento === "altro" ? formData.altroRaffrescamento : formData.raffrescamento;
        stanza.illuminazione = formData.illuminazione === "altro" ? formData.altroIlluminazione : formData.illuminazione;
        stanza.infissi = infissiValues;
        stanze.updateStanza(stanza);

        toast.success("Dati salvati");
        setFormData({
            stanza             : "",
            destinazioneUso    : "",
            piano              : "",
            cappotto           : false,
            altezza            : 0,
            spessoreMuro       : 0,
            riscaldamento      : "",
            altroRiscaldamento : "",
            raffrescamento     : "",
            altroRaffrescamento: "",
            illuminazione      : "",
            altroIlluminazione : ""
        });
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
                    <Select name="stanza"
                            options={ stanzeOptions }
                            onChange={ (newValue) => {
                                handleStanzaChange(newValue);
                            } }
                            className="col-span-4" />
                    <Label htmlFor="destinazioneUso" className="col-span-2"> Destinazione D'uso </Label>
                    <Select name="destinazioneUso"
                            value={ currentValueDestinazioneUso() }
                            onChange={ (newValue: SingleValue<{ value: string, label: string }>) => {
                                setFormData((prev) => ({
                                    ...prev,
                                    destinazioneUso: newValue?.value ?? ""
                                }));
                            } }
                            isDisabled={ destinazioneUsoOptions.length <= 1 }
                            className="col-span-4 rounded-md shadow-sm"
                            options={ destinazioneUsoOptions }
                    />
                </div>
            </div>
            <div className="row-start-2 col-span-12">
                <div className="grid grid-cols-12 items-center gap-5">
                    <Label htmlFor="piano" className="col-span-2">Piano</Label>
                    <Select name="piano"
                            className="col-span-4"
                            options={ pianoOptions }
                            value={ pianoOptions.length === 1 ? pianoOptions[0] : null } // fixme: non mi fa selezionare un elemento perchè mette sempre null
                            isDisabled={ formData.stanza !== "_" }
                            onChange={ (newValue: SingleValue<{ value: string, label: string }>) => {
                                console.log(newValue);
                                setFormData((prev) => ({
                                    ...prev,
                                    piano: newValue?.value ?? ""
                                }));
                            } }
                    />
                    <Label htmlFor="cappotto" className="col-span-2">Cappotto</Label>
                    <div id="cappotto" className="col-span-4 flex items-center justify-start gap-4">
                        <div className="col-span-2 flex items-center gap-2 px-3 py-2">
                            <input type="radio" id="chk_finestra"
                                   name="check_tipo" value="Finestra"
                                   className="h-4 w-4 accent-blue-500"
                                   checked={ formData.cappotto }
                                   onChange={ () => {
                                       setFormData((prev) => ({
                                           ...prev,
                                           cappotto: true
                                       }));
                                   } }
                            />
                            <label htmlFor="chk_finestra">SI</label>
                        </div>
                        <div className="col-span-2 flex items-center gap-2 px-3 py-2">
                            <input type="radio" id="chk_finestra"
                                   name="check_tipo" value="Porta"
                                   className="h-4 w-4 accent-blue-500"
                                   checked={ !formData.cappotto }
                                   onChange={ () => {
                                       setFormData((prev) => ({
                                           ...prev,
                                           cappotto: false
                                       }));
                                   } }
                            />
                            <label htmlFor="chk_porta">NO</label>
                        </div>
                    </div>
                </div>
            </div>
            {/* Altezza */ }
            <div className="row-start-3 col-span-12">
                <div className="grid grid-cols-12 items-center gap-5">
                    <Label htmlFor="altezza" className="col-span-2"> Altezza (cm) </Label>
                    <Input name="altezza"
                           value={ formData.altezza }
                           onChange={ handleInputNumericChange }
                           className="col-span-4"
                    />
                    <Label htmlFor="spessoreMuro" className="col-span-2">Spessore Muro (cm)</Label>
                    <Input name="spessoreMuro"
                           value={ formData.spessoreMuro }
                           onChange={ handleInputNumericChange }
                           className="col-span-4"
                    />
                </div>
            </div>
            {/* Infissi */ }
            <div className="row-start-4 col-span-12">
                <div className="grid grid-cols-12 items-baseline gap-5">
                    <Label htmlFor="infisso" className="col-span-2">Infissi</Label>
                    <div className="col-span-10">
                        <DynamicSelectsInfissi infissiValues={ infissiValues } setInfissiValues={ setInfissiValues } />
                    </div>
                </div>
            </div>
            {/* Riscaldamento */ }
            <div className="row-start-5 col-span-12">
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
            {/* Raffrescamento */ }
            <div className="row-start-6 col-span-12">
                <div className="grid grid-cols-12 items-center gap-5">
                    <Label htmlFor="raffrescamento" className="col-span-2">Raffrescamento</Label>
                    <Select name="raffrescamento"
                            onChange={ (newValue: SingleValue<{ value: string, label: string }>) => {
                                handleSelect(newValue, "raffrescamento");
                            } }
                            className="col-span-4 rounded-md shadow-sm"
                            options={ climatizzazioneTypeOptions }
                    />
                    <Label htmlFor="altroRaffrescamento">Altro</Label>
                    <Input name="altroRaffrescamento" value={ formData.altroRaffrescamento ?? "" }
                           onChange={ handleChange }
                           className="col-span-5 disabled:bg-gray-200" disabled={ altro["raffrescamento"] } />
                </div>
            </div>
            {/* Illuminazione */ }
            <div className="row-start-7 col-span-12">
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
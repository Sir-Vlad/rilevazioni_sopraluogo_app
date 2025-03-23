import { Dispatch, SetStateAction, useState } from "react";
import { faPlus }                             from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon }                    from "@fortawesome/react-fontawesome";
import Select, { SingleValue }                from "react-select";
import Input                                  from "./Input.tsx";
import { useInfissi }                         from "../context/UseProvider.tsx";

interface DynamicSelectsInfissiProps {
    infissiValues: string[];
    setInfissiValues: Dispatch<SetStateAction<string[]>>;
}

const DynamicSelectsInfissi = ({
                                   infissiValues,
                                   setInfissiValues
                               }: DynamicSelectsInfissiProps) => {
    const infissi = useInfissi();
    const [ isOpen, setIsOpen ] = useState(false);
    const [ inputValue, setInputValue ] = useState<number>(0);

    const options = infissi.data.map((item) => {
        if (!item.id) {
            throw new Error("Infisso doesn't have id");
        }
        return {
            value: item.id,
            label: item.id
        };
    });
    const handleSelect = (value: SingleValue<{ value: string, label: string }>, index: number) => {
        setInfissiValues((prev) => {
            if (value !== null) {
                const updateValues = [ ...prev ];
                updateValues[index] = value.value;
                return updateValues;
            }
            return prev;
        });
    };

    const handleCloseModal = () => {
        setIsOpen(false);
        setInputValue(0);
    };

    return <div className="grid grid-cols-4 gap-5 max-h-96">
        { infissiValues.map((_infisso, index) => {
            return <Select key={ index + 1 } options={ options } isSearchable={ true }
                           onChange={ (newValue) => handleSelect(newValue, index) } />;
        }) }
        <div className="flex justify-center items-center">
            <button
                type="button"
                className={ `rounded-full h-6 w-6 flex justify-center items-center
                bg-blue-500 text-white hover:bg-blue-600  ${ infissiValues.length >= 28 ? "hidden" : "" } ` }
                onClick={ () => {
                    setInfissiValues((prev) => [ ...prev, "" ]);
                } }
                onContextMenu={ (e) => {
                    e.preventDefault();
                    setIsOpen(true);
                } }
            >
                <FontAwesomeIcon icon={ faPlus } />
            </button>
        </div>
        {/* Modal */ }
        { isOpen && <div className="fixed inset-0 bg-opacity-50 flex justify-center items-center z-10">
            <div className="bg-white rounded-lg w-3/10 h-52 p-6 border-gray-800 shadow-2xl">
                <h2 className="text-lg font-bold mb-4">Inserisci il numero di finestre da aggiungere</h2>
                <Input name="" value={ inputValue } onChange={ (e) => {
                    const value = parseInt(e.target.value, 0);
                    if (!isNaN(value) && Number(value) < 28 && Number(value) > 0) {
                        setInputValue(Number(value));
                        return;
                    }
                    setInputValue(0);
                } } />
                <p className="text-xs text-gray-500 p-1">
                    <span className="font-bold italic">NB.</span>{ " " }
                    Il numero massimo di finestre aggiungibili è 27
                </p>
                <div className="flex justify-end mt-2">
                    <button
                        type="button"
                        onClick={ handleCloseModal }
                        className="px-4 py-2 bg-gray-500 text-white rounded mr-2"
                    >
                        Annulla
                    </button>
                    <button type="button" className="px-4 py-2 bg-blue-500 text-white rounded" onClick={ () => {
                        const newInfissiValues = Array.from({length: inputValue}, () => "");
                        setInfissiValues((prev) => [ ...prev, ...newInfissiValues ]);
                        handleCloseModal();
                    } }>
                        Inserisci
                    </button>
                </div>
            </div>
        </div> }
    </div>;
};

export default DynamicSelectsInfissi;
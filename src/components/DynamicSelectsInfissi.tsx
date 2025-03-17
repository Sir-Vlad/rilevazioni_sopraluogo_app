import { useInfissi }               from "../context/InfissiProvider.tsx";
import { Dispatch, SetStateAction } from "react";
import { faPlus }                   from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon }          from "@fortawesome/react-fontawesome";
import Select, { SingleValue }      from "react-select";

interface DynamicSelectsInfissiProps {
    infissiValues: string[];
    setInfissiValues: Dispatch<SetStateAction<string[]>>;
}

const DynamicSelectsInfissi = ({
                                   infissiValues,
                                   setInfissiValues
                               }: DynamicSelectsInfissiProps) => {
    const infissi      = useInfissi();
    const options      = infissi.data.map((item) => {
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
                const updateValues  = [ ...prev ];
                updateValues[index] = value.value;
                return updateValues;
            }
            return prev;
        });
    };

    return <div className="grid grid-cols-4 gap-5 max-h-96">
        { infissiValues.map((_infisso, index) => {
            return <Select key={ index + 1 } options={ options } isSearchable={ true }
                           onChange={ (newValue) => handleSelect(newValue, index) } />;
        }) }
        <div className="flex justify-center items-center">
            <button
                className={ `rounded-full h-6 w-6 flex justify-center items-center
                bg-blue-500 text-white hover:bg-blue-600  ${ infissiValues.length >= 50 ? "hidden" : "" } ` }
                onClick={ (e) => {
                    e.preventDefault();
                    setInfissiValues((prev) => [ ...prev, "" ]);
                } }>
                <FontAwesomeIcon icon={ faPlus } />
            </button>
        </div>
    </div>;
};

export default DynamicSelectsInfissi;
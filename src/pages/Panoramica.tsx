import Select                from "react-select";
import { useState }          from "react";
import DynamicSelectsInfissi from "../components/DynamicSelectsInfissi.tsx";

const options: { value: string, label: string }[] = [
    {
        value: "chocolate",
        label: "Chocolate"
    },
    {
        value: "strawberry",
        label: "Strawberry"
    },
    {
        value: "vanilla",
        label: "Vanilla"
    }
];


const Panoramica = () => {
    const [ selectedOption, setSelectedOption ] = useState("");
    const [ infissiValues, setInfissiValues ]   = useState<string[]>([ "" ]);

    return (<div className="flex items-center justify-center">
        <Select
            name="color"
            defaultValue={ options[0] }
            options={ options }
            isSearchable={ true }
        />
        <DynamicSelectsInfissi infissiValues={ infissiValues } setInfissiValues={ setInfissiValues } />
    </div>);
};

export default Panoramica;
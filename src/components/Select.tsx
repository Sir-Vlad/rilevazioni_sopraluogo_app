import * as React from "react";

interface SelectProps {
    name: string;
    value: string | number;
    onChange: (e: React.ChangeEvent<HTMLSelectElement>) => void;
    className?: string;
    optionAltro?: boolean;
    children: React.ReactNode;
}


const Select = ({
                    name,
                    value,
                    onChange,
                    className,
                    optionAltro = false,
                    children
                }: SelectProps) => {
    return <select
        id={ name }
        name={ name }
        value={ value }
        onChange={ onChange }
        className={ `w-full px-3 py-2 bg-white border border-gray-300 rounded-md shadow-sm appearance-none
            focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 ${ className ?? "" }` }
    >
        <option value="" disabled={ true } hidden={ true }>Seleziona ...</option>
        { children }
        { optionAltro && <option value="altro">Altro</option> }
    </select>;
};

export default Select;





















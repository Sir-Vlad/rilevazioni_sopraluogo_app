import * as React from "react";

interface InputProps {
    name: string;
    value: string | number;
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    placeholder?: string;
    className?: string;
    disabled?: boolean;
}


const Input = ({
                   name,
                   value,
                   onChange,
                   placeholder,
                   className = "",
                   disabled = false
               }: InputProps) => {
    return <input
        type="text"
        id={ name }
        name={ name }
        value={ value }
        onChange={ onChange }
        className={ `w-full px-3 py-2 bg-white border border-gray-300 rounded-md shadow-sm 
        focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 ${ className }` }
        placeholder={ placeholder }
        disabled={ disabled }
    />;
};

export default Input;
import { ChangeEvent } from "react";

export const capitalize = (str: string) => {
    return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase();
};

export const getFileName = (filePath: string): string => {
    const fileNameWithExt = filePath.split(/[/\\]/).pop() ?? "";
    return fileNameWithExt.split(".").slice(0, -1).join(".");
};

export const handleInputNumericChange = <T extends (value: number) => void>(event: ChangeEvent<HTMLInputElement>, onChange: T) => {
    const {value} = event.target;
    if (value.length === 0) {
        onChange(0);
        return;
    }
    if (/^\D$/.test(value[value.length - 1])) {
        const newValue = value.slice(0, -1);
        if (newValue.length === 0) {
            onChange(0);
        } else {
            onChange(Number(newValue));
        }
        return;
    }
    onChange(Number(value));
};
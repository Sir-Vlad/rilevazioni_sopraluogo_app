import { ChangeEvent } from "react";

export const capitalize = (str: string) => {
    return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase();
};

export const sanitizeString = (str: string) => {
    return str.replace(/_/g, " ");
};

export const getFileNameWithExtension = (filePath: string) => {
    return filePath.split(/[/\\]/).pop() ?? "";
};

export const getFileName = (filePath: string): string => {
    const fileNameWithExt = getFileNameWithExtension(filePath);
    return fileNameWithExt.split(".")[0];
};


export const handleInputNumericChange = <T extends (value: number) => void>(event: ChangeEvent<HTMLInputElement>, onChange: T) => {
    const input = event.target;
    const {
        value,
        selectionStart
    } = input;
    if (value.length === 0) {
        onChange(0);
        return;
    }
    const cursorPosition = selectionStart ?? 0;
    const valueBeforeCursor = value.substring(0, cursorPosition);
    const nonNumericBeforeCursor = (valueBeforeCursor.match(/\D/g) || []).length;

    const newValue = value.replace(/\D/g, "");
    if (newValue.length === 0) {
        onChange(0);
    } else {
        onChange(parseInt(newValue));
    }

    setTimeout(() => {
        const newCursorPosition = Math.max(0, cursorPosition - nonNumericBeforeCursor);
        input.setSelectionRange(newCursorPosition, newCursorPosition);
    }, 0);
};
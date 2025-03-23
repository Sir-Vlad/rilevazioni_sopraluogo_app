export const capitalize = (str: string) => {
    return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase();
};

export const getFileName = (filePath: string): string => {
    const fileNameWithExt = filePath.split(/[/\\]/).pop() ?? "";
    return fileNameWithExt.split(".").slice(0, -1).join(".");
};

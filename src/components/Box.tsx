import * as React from "react";

interface BoxProps {
    className?: string;
    children: React.ReactNode;
}


const Box = ({
                 className,
                 children
             }: BoxProps) => {
    return <div className={ ` rounded-lg bg-gray-200 shadow-gray-950 p-4 ${ className ?? "" }` }>
        { children }
    </div>;
};

export default Box;
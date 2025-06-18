import { Input } from "@/components/ui/input.tsx";
import * as React from "react";

const InputWithMeasureUnit = ({
                                  unitLabel,
                                  ...args
                              }: { unitLabel: string } & React.ComponentProps<"input">) => {
    return <div className="relative">
        <Input { ...args }/>
        <span
            className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 text-sm">{ unitLabel }</span>
    </div>
}

export default InputWithMeasureUnit;
import { defineConfig } from "vite";
import react            from "@vitejs/plugin-react";
import tailwindcss      from "@tailwindcss/vite";
import path             from "path";


// https://vite.dev/config/
export default defineConfig({
    plugins    : [ react(), tailwindcss() ],
    resolve    : {
        alias: {
            "@": path.resolve(__dirname, "src")
        }
    },
    clearScreen: false,
    server     : {
        strictPort: true,
        watch     : {
            ignored: [ "**/src-tauri/**" ]
        }
    },
    envPrefix  : [ "VITE_", "TAURI_" ],
    build      : {
        target   : process.env.TAURI_ENV_PLATFORM == "windows" ? "chrome105" : "safari13",
        minify   : !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
        sourcemap: !!process.env.TAURI_ENV_DEBUG
    }
});

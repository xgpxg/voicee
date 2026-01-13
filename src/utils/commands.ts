import {invoke} from "@tauri-apps/api/core";
import {ElMessage} from "element-plus";

const IS_DEV = import.meta.env.VITE_IS_DEV

const isDev = () => {
    return IS_DEV === 1 || IS_DEV === '1'
}

const call = async <T = any>(command: string, args: any, options?: { hideError?: boolean }): Promise<T> => {
    try {
        if (isDev()) {
            console.log(command, args)
        }
        return await invoke(command, args)
    } catch (e: any) {
        if (!options?.hideError) {
            ElMessage.error(e.message || e)
        }
        throw e
    }

}

export {
    call,
    isDev
}
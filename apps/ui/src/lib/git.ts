import { commands } from "@gitultra/schemas/ts/gitultra/bindings";
import { open } from "@tauri-apps/plugin-dialog";

export async function loadRepo() {
    const dir = await open({
        multiple: false,
        directory: true
    });

    console.info("Opening repo in directory: ", dir);

    if (!dir){
        console.error("Invalid path: " + dir)
        return;
    }

    return await commands.openRepoDirectory(dir);
}
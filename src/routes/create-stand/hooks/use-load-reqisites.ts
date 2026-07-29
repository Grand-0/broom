import { useEffect, useState } from "react";
import { ICreateStandRequisites } from "../../../models";
import { invoke } from "@tauri-apps/api/core";

export function useLoadRequisites() {
    const [requisites, setRequisites] = useState<ICreateStandRequisites | Error | undefined>();

    useEffect(() => {
        (async () => {
            try {
                const requisites = await invoke<ICreateStandRequisites>(
                    "get_create_stand_form_data"
                );

                setRequisites(requisites);
            } catch (error: unknown) {
                if (typeof error === "string") {
                    setRequisites(new Error(error));
                }
            }
        })();
    }, []);

    return { requisites };
}

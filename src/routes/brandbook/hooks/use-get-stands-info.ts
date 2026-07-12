import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { IStandInfo } from "../../../models";

export function useStandsInfo() {
  const [result, setResult] = useState<IStandInfo[] | Error | undefined>();

  useEffect(() => {
    (async () => {
      try {
        const standsInfo = await invoke<IStandInfo[]>("get_user_stands_info");

        setResult(standsInfo);
      } catch (error: unknown) {
        if (typeof error === "string") {
          setResult(new Error(error));
        }
      }
    })();
  });

  return { result };
}

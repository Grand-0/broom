import { ICreateStandError, ILogInfo } from "../../../models";

export type SubmitError = {
    message: string;
    logInfo?: ILogInfo | null;
};

export function createSubmitError(error: unknown): SubmitError {
    if (error && typeof error === "object" && "errorMsg" in error) {
        const e = error as ICreateStandError;
        return { message: e.errorMsg, logInfo: e.logInfo ?? null };
    }
    return { message: typeof error === "string" ? error : "Неизвестная ошибка", logInfo: null };
}

export interface ICreateStandRequest {
    collection: string;
    project: string;
    version: string;
    stage: string;
    buildOption: string;
    buildVersion: string;
    useElastic: boolean;
    useKafka: boolean;
    tempFilesPath?: string;
}

export type ILogType = "session" | "operation";

export interface ILogInfo {
    logType: ILogType;
    logName: string;
}

export interface ICreateStandResponse {
    status: string;
    logInfo?: ILogInfo | null;
}

export interface ICreateStandError {
    errorMsg: string;
    logInfo?: ILogInfo | null;
}

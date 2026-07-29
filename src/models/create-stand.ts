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

export interface ICreateStandResponse {
    status: string;
    logPath?: string | null;
}

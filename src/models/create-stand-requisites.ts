export interface ICreateStandRequisites {
   collections: ICollectionFormItem[],
}

export interface ICollectionFormItem {
  collectionName: string;
  projects: IProjectFormItem[],
}

export interface IProjectFormItem {
  projectName: string;
  versions: IVersionFormItem[],
}

export interface IVersionFormItem {
  version: string;
  stages: IStageFormItem[];
}

export interface IStageFormItem {
  stageName: string;
}

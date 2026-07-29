import { create } from "zustand";
import { getDropdownOptions } from "../helpers";
import { BuildRadioValue } from "../types";

import { ICreateStandRequisites } from "../../../models";

import { DropdownOption } from "../../../components/dropdown";
import { RadioOption } from "../../../components/radio-button";

interface IFormCreateStandStore {
    formRequisites: ICreateStandRequisites | null;

    useElastic: boolean;
    useKafka: boolean;

    tempPath?: string;

    collectionOptions: DropdownOption[];
    projectOptions: DropdownOption[];
    versionOptions: DropdownOption[];
    stageOptions: DropdownOption[];
    buildOptions: RadioOption<BuildRadioValue>[];

    buildOption: BuildRadioValue;
    collectionOption?: string;
    projectOption?: string;
    versionOption?: string;
    stageOption?: string;

    buildVersion: "latest" | string;

    changeCollection: (value: string) => void;
    changeProject: (value: string) => void;
    changeVersion: (value: string) => void;
    changeStage: (value: string) => void;

    setFormRequisites: (formRequisites: ICreateStandRequisites) => void;

    clear: () => void;
}

export const useFormCreateStandStore = create<IFormCreateStandStore>((set, get) => ({
    formRequisites: null,

    useElastic: true,
    useKafka: true,

    tempPath: undefined,

    collectionOptions: [],
    projectOptions: [],
    versionOptions: [],
    stageOptions: [],
    buildOptions: [
        { label: "Latest issue", value: "latest" },
        // TODO: release 1.1.0
        // { label: "Select an issue", value: "specific" },
    ],

    buildVersion: "latest",

    collectionOption: undefined,
    projectOption: undefined,
    versionOption: undefined,
    stageOption: undefined,
    buildOption: "latest",

    changeCollection: (value) => {
        if (value === get().collectionOption) return;

        const formRequisites = get().formRequisites;

        if (!formRequisites) return;

        const selectedCollection = getCollection(formRequisites, value);

        if (!selectedCollection) return;

        set({
            projectOptions: getDropdownOptions(
                selectedCollection.projects.map((p) => p.projectName)
            ),
            versionOptions: [],
            stageOptions: [],

            collectionOption: value,
            projectOption: undefined,
            versionOption: undefined,
            stageOption: undefined,

            buildOption: "latest",
            buildVersion: undefined,
        });
    },
    changeProject: (value) => {
        if (value === get().projectOption) return;

        const formRequisites = get().formRequisites;

        if (!formRequisites) return;

        const collection = get().collectionOption;

        if (!collection) return;

        const selectedProject = getProject(formRequisites, collection, value);

        if (!selectedProject) return;

        set({
            versionOptions: getDropdownOptions(selectedProject.versions.map((v) => v.version)),
            stageOptions: [],

            projectOption: value,
            versionOption: undefined,
            stageOption: undefined,

            buildOption: "latest",
            buildVersion: undefined,
        });
    },
    changeVersion: (value) => {
        if (value === get().versionOption) return;

        const formRequisites = get().formRequisites;

        if (!formRequisites) return;

        const collection = get().collectionOption;

        if (!collection) return;

        const selectedProject = get().projectOption;

        if (!selectedProject) return;

        const version = getVersion(formRequisites, collection, selectedProject, value);

        if (!version) return;

        set({
            stageOptions: getDropdownOptions(version.stages.map((s) => s.stageName)),

            versionOption: value,
            stageOption: undefined,

            buildOption: "latest",
            buildVersion: undefined,
        });
    },
    changeStage: (value) => {
        if (value === get().stageOption) return;

        set({
            stageOption: value,

            buildOption: "latest",
            buildVersion: undefined,
        });
    },

    setFormRequisites: (formRequisites) => {
        set({
            formRequisites: formRequisites,
            collectionOptions: getDropdownOptions(
                formRequisites.collections.map((c) => c.collectionName)
            ),
        });
    },

    clear: () => {
        set({
            formRequisites: null,

            collectionOptions: [],
            projectOptions: [],
            versionOptions: [],
            stageOptions: [],

            collectionOption: undefined,
            projectOption: undefined,
            versionOption: undefined,
            stageOption: undefined,

            buildOption: "latest",
            buildVersion: "latest",

            useElastic: true,
            useKafka: true,

            tempPath: undefined,
        });
    },
}));

function getCollection(formRequisites: ICreateStandRequisites, collectionName: string) {
    return formRequisites.collections.find((c) => c.collectionName === collectionName);
}

function getProject(
    formRequisites: ICreateStandRequisites,
    collectionName: string,
    projectName: string
) {
    const collection = getCollection(formRequisites, collectionName);

    if (!collection) return;

    return collection.projects.find((p) => p.projectName === projectName);
}

function getVersion(
    formRequisites: ICreateStandRequisites,
    collectionName: string,
    projectName: string,
    version: string
) {
    const project = getProject(formRequisites, collectionName, projectName);

    if (!project) return;

    return project.versions.find((v) => v.version === version);
}

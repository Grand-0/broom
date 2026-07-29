import { useEffect, useState } from "react";

import { Dropdown } from "../../components/dropdown";
import { RadioGroup } from "../../components/radio-button";
import { Checkbox } from "../../components/checkbox";
import { TextInput } from "../../components/text-input";
import { ErrorCard } from "../../components/error-card/error-card";

import { ICreateStandRequisites } from "../../models";

import { useFormCreateStandStore } from "./stores/form";
import { useLoadRequisites } from "./hooks/use-load-reqisites";

import "./create-stand.scss";

export function CreateStandContent() {
    const { requisites } = useLoadRequisites();

    if (typeof requisites === "undefined") {
        // TODO: спиннер
        return <></>;
    }

    if (requisites instanceof Error) {
        return (
            <div className="content">
                <ErrorCard message={requisites.message} />
            </div>
        );
    }

    return (
        <div className="content">
            <CreateStandContentInner requisites={requisites} />
        </div>
    );
}

function CreateStandContentInner({ requisites }: { requisites: ICreateStandRequisites }) {
    const {
        collectionOptions,
        buildOptions,
        projectOptions,
        stageOptions,
        versionOptions,
        useElastic,
        useKafka,
        buildOption,
        buildVersion,
        collectionOption,
        projectOption,
        stageOption,
        versionOption,
        tempPath,
    } = useFormCreateStandStore((s) => s);

    useEffect(() => {
        useFormCreateStandStore.getState().setFormRequisites(requisites);
    }, [requisites]);

    useEffect(() => {
        return () => {
            useFormCreateStandStore.getState().clear();
        };
    }, []);

    const [additionalOpen, setAdditionalOpen] = useState(false);

    const canSubmit =
        collectionOption &&
        projectOption &&
        versionOption &&
        stageOption &&
        (buildOption === "latest" || buildVersion);

    return (
        <div className="create-stand-card">
            <div className="create-stand-fields-row">
                <div className="create-stand-column">
                    <Dropdown
                        label="Collection"
                        placeholder="Select collection"
                        options={collectionOptions}
                        value={collectionOption}
                        onChange={(value) => {
                            useFormCreateStandStore.getState().changeCollection(value);
                        }}
                    />
                    <Dropdown
                        label="Version"
                        placeholder="Select version"
                        options={versionOptions}
                        value={versionOption}
                        onChange={(value) => {
                            useFormCreateStandStore.getState().changeVersion(value);
                        }}
                    />
                    <RadioGroup
                        label="Build"
                        options={buildOptions}
                        value={buildOption}
                        onChange={(value) => {
                            useFormCreateStandStore.setState({ buildOption: value });
                        }}
                    />
                </div>
                <div className="create-stand-column">
                    <Dropdown
                        label="Project"
                        placeholder="Select project"
                        options={projectOptions}
                        value={projectOption}
                        onChange={(value) => {
                            useFormCreateStandStore.getState().changeProject(value);
                        }}
                    />
                    <Dropdown
                        label="Stage"
                        placeholder="Select stage"
                        options={stageOptions}
                        value={stageOption}
                        onChange={(value) => {
                            useFormCreateStandStore.getState().changeStage(value);
                        }}
                    />

                    {buildOption === "specific" && (
                        <Dropdown
                            label="Build Version"
                            options={[]}
                            value={buildVersion}
                            onChange={(value) => {
                                useFormCreateStandStore.setState({ buildVersion: value });
                            }}
                            placeholder="—  Select version  —"
                        />
                    )}
                </div>
            </div>

            <div className="create-stand-divider" />

            <button
                className="create-stand-additional-header"
                onClick={() => setAdditionalOpen((v) => !v)}
                type="button"
            >
                <span className={`create-stand-additional-arrow${additionalOpen ? " open" : ""}`}>
                    ▶
                </span>
                <span className="create-stand-additional-title">Дополнительно</span>
                <span className={`create-stand-additional-hint${additionalOpen ? " hidden" : ""}`}>
                    ElasticSearch, Kafka, TempFilesPath
                </span>
            </button>

            <div className={`create-stand-additional-body${additionalOpen ? " open" : ""}`}>
                <div className="create-stand-fields-row">
                    <div className="create-stand-column">
                        <Checkbox
                            label="ElasticSearch"
                            checked={useElastic}
                            onChange={(checked) =>
                                useFormCreateStandStore.setState({ useElastic: checked })
                            }
                        />
                        <Checkbox
                            label="Kafka"
                            checked={useKafka}
                            onChange={(checked) =>
                                useFormCreateStandStore.setState({ useKafka: checked })
                            }
                        />
                    </div>
                    <div className="create-stand-column">
                        <TextInput
                            label="TempFilesPath"
                            value={tempPath}
                            onChange={(value) => {
                                useFormCreateStandStore.setState({ tempPath: value });
                            }}
                            placeholder="$env:TEMP\\deploy_stand"
                        />
                    </div>
                </div>
            </div>

            <div className="create-stand-actions">
                <button
                    className="create-stand-btn create-stand-btn-primary"
                    type="submit"
                    disabled={!canSubmit}
                >
                    Create
                </button>
                <button className="create-stand-btn create-stand-btn-ghost" type="button">
                    Cancel
                </button>
            </div>
        </div>
    );
}

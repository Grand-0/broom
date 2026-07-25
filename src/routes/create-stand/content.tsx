import { useState } from "react";
import { Dropdown } from "../../components/dropdown";
import { RadioGroup } from "../../components/radio-button";
import { Checkbox } from "../../components/checkbox";
import { TextInput } from "../../components/text-input";

import "./create-stand.scss";

const collections = [
    { label: "Delo2020", value: "Delo2020" },
    { label: "Archive2020", value: "Archive2020" },
];

const versions: Record<string, { label: string; value: string }[]> = {
    Delo2020: [{ label: "26.2", value: "26.2" }],
    Archive2020: [{ label: "1.0", value: "1.0" }],
};

const branches: Record<string, { label: string; value: string }[]> = {
    "26.2": [
        { label: "Основной", value: "main" },
        { label: "Бизнес Васи Пупкина", value: "biz_pupkin" },
    ],
    "1.0": [{ label: "Основной", value: "main" }],
};

const stages = [
    { label: "Dev", value: "dev" },
    { label: "Release", value: "release" },
    { label: "Prod", value: "prod" },
];

const buildOptions = [
    { label: "Последний выпуск", value: "latest" },
    { label: "Выбрать выпуск", value: "specific" },
];

const buildVersions: Record<string, { label: string; value: string }[]> = {
    "26.2": [
        { label: "26.2.21045", value: "26.2.21045" },
        { label: "26.2.21032", value: "26.2.21032" },
        { label: "26.2.21018", value: "26.2.21018" },
    ],
    "1.0": [
        { label: "1.0.1001", value: "1.0.1001" },
        { label: "1.0.998", value: "1.0.998" },
    ],
};

export function CreatestandContent() {
    const [collection, setCollection] = useState("");
    const [version, setVersion] = useState("");
    const [branch, setBranch] = useState("");
    const [stage, setStage] = useState("");
    const [build, setBuild] = useState("latest");
    const [buildVersion, setBuildVersion] = useState("");
    const [useElastic, setUseElastic] = useState(false);
    const [useKafka, setUseKafka] = useState(false);
    const [tempPath, setTempPath] = useState("");
    const [additionalOpen, setAdditionalOpen] = useState(false);

    const currentVersions = collection ? (versions[collection] ?? []) : [];
    const currentBranches = version ? (branches[version] ?? []) : [];
    const currentBuildVersions = version ? (buildVersions[version] ?? []) : [];

    const canSubmit = collection && version && branch && stage && (build === "latest" || buildVersion);

    return (
        <div className="content">
            <div className="create-stand-card">
                <div className="create-stand-fields-row">
                    <div className="create-stand-column">
                        <Dropdown
                            label="Коллекция"
                            options={collections}
                            value={collection}
                            onChange={(v) => {
                                setCollection(v);
                                setVersion("");
                                setBranch("");
                            }}
                        />
                        <Dropdown
                            label="Версия"
                            options={currentVersions}
                            value={version}
                            onChange={(v) => {
                                setVersion(v);
                                setBranch("");
                            }}
                        />
                        <Dropdown
                            label="Ветка"
                            options={currentBranches}
                            value={branch}
                            onChange={setBranch}
                        />
                    </div>
                    <div className="create-stand-column">
                        <Dropdown
                            label="Стадия"
                            options={stages}
                            value={stage}
                            onChange={setStage}
                        />
                        <RadioGroup
                            label="Сборка"
                            options={buildOptions}
                            value={build}
                            onChange={setBuild}
                        />
                        {build === "specific" && (
                            <Dropdown
                                label="Версия выпуска"
                                options={currentBuildVersions}
                                value={buildVersion}
                                onChange={setBuildVersion}
                                placeholder="—  Выберите версию  —"
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
                    <span
                        className={`create-stand-additional-arrow${additionalOpen ? " open" : ""}`}
                    >
                        ▶
                    </span>
                    <span className="create-stand-additional-title">Дополнительно</span>
                    <span
                        className={`create-stand-additional-hint${additionalOpen ? " hidden" : ""}`}
                    >
                        ElasticSearch, Kafka, TempFilesPath
                    </span>
                </button>

                <div className={`create-stand-additional-body${additionalOpen ? " open" : ""}`}>
                    <div className="create-stand-fields-row">
                        <div className="create-stand-column">
                            <Checkbox
                                label="ElasticSearch"
                                checked={useElastic}
                                onChange={setUseElastic}
                            />
                            <Checkbox label="Kafka" checked={useKafka} onChange={setUseKafka} />
                        </div>
                        <div className="create-stand-column">
                            <TextInput
                                label="TempFilesPath"
                                value={tempPath}
                                onChange={setTempPath}
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
                        Создать
                    </button>
                    <button className="create-stand-btn create-stand-btn-ghost" type="button">
                        Отмена
                    </button>
                </div>
            </div>
        </div>
    );
}

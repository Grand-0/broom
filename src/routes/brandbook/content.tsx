import { StandCard } from "../../components/stand-card";
import { useStandsInfo } from "./hooks/use-get-stands-info";
import { AdditionInfoContentPart } from "./addition-info";

import "./brandbook.scss";
import { IStandInfo } from "../../models";

export function BrandBookContent() {
    const { result } = useStandsInfo();

    if (typeof result === "undefined") {
        // TODO: спиннер
        return <></>;
    }

    return (
        <div className="content">
            <div className="brandbook-header">
                <button className="add-stand-btn">
                    <p>+ Add Stand</p>
                </button>
            </div>
            <BrandBookContentInner viewContent={result} />
        </div>
    );
}

function BrandBookContentInner({ viewContent }: { viewContent: Error | IStandInfo[] }) {
    if (viewContent instanceof Error) {
        return <AdditionInfoContentPart errorMessage={viewContent.message} />;
    }

    return !!viewContent.length ? (
        <div className="brandbook-stand-list">
            {viewContent.map((s) => (
                <StandCard standInfo={s} />
            ))}
        </div>
    ) : (
        <AdditionInfoContentPart />
    );
}

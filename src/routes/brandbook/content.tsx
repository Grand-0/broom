import { StandCard } from "../../components/stand-card";
import { useStandsInfo } from "./hooks/use-get-stands-info";
import { AdditionInfoContentPart } from "./addition-info";
import { IStandInfo } from "../../models";

import "./brandbook.scss";
import { useNavigate } from "react-router-dom";
import { RouteNames } from "../sources";

export function BrandBookContent() {
    const navigate = useNavigate();
    const { result } = useStandsInfo();

    if (typeof result === "undefined") {
        // TODO: спиннер
        return <></>;
    }

    return (
        <div className="content">
            <div className="brandbook-header">
                <button
                    className="add-stand-btn"
                    onClick={() => {
                        navigate(`/${RouteNames.CreateStand}`);
                    }}
                >
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

import { StandCard } from "../../components/stand-card";
import { useStandsInfo } from "./hooks/use-get-stands-info";
import { AdditionInfoContentPart } from "./addition-info";
import { IStandInfo } from "../../models";

import "./brandbook.scss";
import { useNavigate } from "react-router-dom";
import { RouteNames } from "../sources";
import { ErrorCard } from "../../components/error-card/error-card";

export function BrandBookContent() {
    const navigate = useNavigate();

    const { result } = useStandsInfo();

    if (typeof result === "undefined") {
        // TODO: спиннер
        return <></>;
    }

    if (result instanceof Error) {
        return (
            <div className="content">
                <ErrorCard message={result.message} />{" "}
            </div>
        );
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

function BrandBookContentInner({ viewContent }: { viewContent: IStandInfo[] }) {
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

import { BroomIcon } from "../../components/icons";

import "./brandbook.scss";

type AdditionInfoContentPartProps = {
    errorMessage?: string;
};

export function AdditionInfoContentPart({ errorMessage }: AdditionInfoContentPartProps) {
    return (
        <div className="brandbook-empty-state">
            <BroomIcon className="brandbook-empty-icon" />
            {errorMessage ? (
                <h2 className="brandbook-empty-title">{errorMessage}</h2>
            ) : (
                <>
                    <h2 className="brandbook-empty-title">No stands yet</h2>
                    <p className="brandbook-empty-subtitle">
                        Create your first stand to get started
                    </p>
                </>
            )}
        </div>
    );
}

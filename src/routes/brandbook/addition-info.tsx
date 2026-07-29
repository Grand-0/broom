import { BroomIcon } from "../../components/icons";

import "./brandbook.scss";

export function AdditionInfoContentPart() {
    return (
        <div className="brandbook-empty-state">
            <BroomIcon className="brandbook-empty-icon" />
            <h2 className="brandbook-empty-title">No stands yet</h2>
            <p className="brandbook-empty-subtitle">Create your first stand to get started</p>
        </div>
    );
}

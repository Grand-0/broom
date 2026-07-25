import { DevelopingIcon } from "../../components/icons";
import "./developing.scss";

type DevelopingContentProps = {
    estimatedVersion: string;
};

export function DevelopingContent({ estimatedVersion }: DevelopingContentProps) {
    return (
        <div className="content">
            <div className="developing-card">
                <div className="developing-icon-container">
                    <DevelopingIcon />
                </div>
                <span className="developing-title">Section in Development</span>
                <span className="developing-subtitle">
                    This section is currently being built and will be available soon
                </span>
                <div className="developing-progress-bar" />
                <span className="developing-status">Estimated completion: {estimatedVersion}</span>
            </div>
        </div>
    );
}

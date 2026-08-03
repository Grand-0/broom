import { CloseIcon, DocumentIcon } from "../icons";
import "./error-card.scss";

type ErrorCardAction = {
    label: string;
    onClick: () => void;
    fileName?: string;
};

type ErrorCardProps = {
    message: string;
    action?: ErrorCardAction;
};

export function ErrorCard({ message, action }: ErrorCardProps) {
    return (
        <div className="error-card">
            <CloseIcon className="error-card-icon" />
            <div className="error-card-title">Application Error</div>
            <div className="error-card-message">{message}</div>
            {action && (
                <button className="error-card-action" type="button" onClick={action.onClick}>
                    <DocumentIcon className="error-card-action-icon" />
                    <span className="error-card-action-label">{action.label}</span>
                    {action.fileName && (
                        <span className="error-card-action-file">{action.fileName}</span>
                    )}
                </button>
            )}
        </div>
    );
}

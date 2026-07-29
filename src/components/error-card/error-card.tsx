import { CloseIcon } from "../icons";
import "./error-card.scss";

type ErrorCardProps = {
    message: string;
};

export function ErrorCard({ message }: ErrorCardProps) {
    return (
        <div className="error-card">
            <CloseIcon className="error-card-icon" />
            <div className="error-card-title">Application Error</div>
            <div className="error-card-message">{message}</div>
        </div>
    );
}

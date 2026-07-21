import { useLocation } from "react-router-dom";
import { getTitle } from "./title";

export function Header() {
    const location = useLocation();

    const title = getTitle(location);

    if (!title) return null;

    return (
        <div className="header">
            <div className="hamburger"></div>
            <h4>{title}</h4>
        </div>
    );
}

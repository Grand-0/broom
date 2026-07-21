import { useLocation } from "react-router-dom";
import { getTitle } from "./title";
import { Hamburger } from "../../components/hamburger";

import "./header.scss";

type HeaderProps = {
    onToggleSidebar: () => void;
    sidebarCollapsed: boolean;
};

export function Header({ onToggleSidebar, sidebarCollapsed }: HeaderProps) {
    const location = useLocation();

    const title = getTitle(location);

    if (!title) return null;

    return (
        <div className="header">
            <Hamburger active={!sidebarCollapsed} onClick={onToggleSidebar} />
            <h4>{title}</h4>
        </div>
    );
}

import { useCallback } from "react";
import { matchPath, useLocation, useNavigate } from "react-router-dom";

type NavigationCardProps = {
    title: string;
    icon: React.ReactElement;
    toPath: string;
};

export function NavigationCard({ icon, title, toPath }: NavigationCardProps) {
    const navigate = useNavigate();
    const location = useLocation();

    const isActive = matchPath({ path: toPath }, location.pathname) !== null;

    const onNavigate = useCallback(() => {
        navigate(toPath);
    }, [navigate]);

    return (
        <div
            className={isActive ? "navigation-card-active" : "navigation-card"}
            onClick={onNavigate}
        >
            {icon}
            <h5>{title}</h5>
        </div>
    );
}

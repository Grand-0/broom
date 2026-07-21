import { BrandbookIcon, BroomIcon, InfoIcon, SettingsIcon } from "../../components/icons";
import { NavigationCard } from "../../components/navigation-card";
import { RouteNames } from "../../routes/sources";

import "./sidebar.scss";

type SiderProps = {
    collapsed?: boolean;
};

export function Sider({ collapsed = false }: SiderProps) {
    return (
        <div className={`sidebar${collapsed ? " collapsed" : ""}`}>
            <div className="logo">
                <BroomIcon />
                {!collapsed && <h3>Broom</h3>}
            </div>

            <div className="navigation">
                <NavigationCard
                    icon={<BrandbookIcon />}
                    toPath={`/${RouteNames.BrandBook}`}
                    title="Brandbook"
                    collapsed={collapsed}
                />
                <NavigationCard
                    icon={<SettingsIcon />}
                    toPath={`/${RouteNames.Settings}`}
                    title="Settings"
                    collapsed={collapsed}
                />
                <NavigationCard
                    icon={<InfoIcon />}
                    toPath={`/${RouteNames.About}`}
                    title="Info"
                    collapsed={collapsed}
                />
            </div>
        </div>
    );
}

import { BrandbookIcon, BroomIcon, InfoIcon, SettingsIcon } from "../../components/icons";
import { NavigationCard } from "../../components/navigation-card";
import { RouteNames } from "../../routes/sources";

import "./sidebar.scss";

export function Sider() {
    return (
        <div className="sidebar">
            <div className="logo">
                {/*<img src="/icons/broom.svg" />*/}
                <BroomIcon />
                <h3>Broom</h3>
            </div>

            <div className="navigation">
                <NavigationCard
                    icon={<BrandbookIcon />}
                    toPath={`/${RouteNames.BrandBook}`}
                    title="Brandbook"
                />
                <NavigationCard
                    icon={<SettingsIcon />}
                    toPath={`/${RouteNames.Settings}`}
                    title="Settings"
                />
                <NavigationCard icon={<InfoIcon />} toPath={`/${RouteNames.About}`} title="Info" />
            </div>
        </div>
    );
}

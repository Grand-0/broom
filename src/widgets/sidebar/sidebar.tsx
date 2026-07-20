import { NavigationCard } from "../../components/navigation-card";
import { RouteNames } from "../../routes/sources";

export function Sider() {
    return (
        <div className="sidebar">
            <div className="logo">
                <img src="/icons/broom.svg" />
                <h3>Broom</h3>
            </div>

            <div className="navigation">
                <NavigationCard
                    imagePath="/icons/brandbook.svg"
                    toPath={`/${RouteNames.BrandBook}`}
                    title="Brandbook"
                />
                <NavigationCard
                    imagePath="/icons/settings.svg"
                    toPath={`/${RouteNames.Settings}`}
                    title="Settings"
                />
                <NavigationCard
                    imagePath="/icons/info.svg"
                    toPath={`/${RouteNames.About}`}
                    title="Info"
                />
            </div>
        </div>
    );
}

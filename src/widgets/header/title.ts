import { matchPath, Location } from "react-router-dom";
import { RouteNames } from "../../routes";

export function getTitle(location: Location) {
    const routes: RouteNames[] = [
        RouteNames.BrandBook,
        RouteNames.CreateStand,
        RouteNames.About,
        RouteNames.Settings,
    ];

    const currentRoute = routes.find((r) => matchPath({ path: r }, location.pathname));

    if (!currentRoute) {
        return null;
    }

    return currentRoute;
}

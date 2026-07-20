import { matchPath, Route, Routes, useLocation, Location } from "react-router-dom";
import { Header } from "../header";
import { RouteNames } from "../../routes/sources";
import { lazy } from "react";

const AboutContent = lazy(() => import("../../routes/about/content"));

export function RightPanel() {
    const location = useLocation();

    return (
        <div className="right-panel">
            <Header title={getTitle(location)} />
            <Routes>
                <Route path="/" element={<></>} />
                <Route path={`/${RouteNames.BrandBook}`} element={<></>} />
                <Route path={`/${RouteNames.CreateStand}`} element={<></>} />
                <Route path={`/${RouteNames.About}`} element={<AboutContent />} />
                <Route path={`/${RouteNames.Settings}`} element={<></>} />
            </Routes>
        </div>
    );
}

function getTitle(location: Location) {
    const routes: RouteNames[] = [
        RouteNames.Empty,
        RouteNames.BrandBook,
        RouteNames.CreateStand,
        RouteNames.About,
        RouteNames.Settings,
    ];

    const currentRoute = routes.find((r) => matchPath({ path: r }, location.pathname));

    if (!currentRoute) {
        throw new Error(`Cannot find route for path - ${location.pathname}`);
    }

    return currentRoute;
}

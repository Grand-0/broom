import { Route, Routes, Navigate } from "react-router-dom";
import { Header } from "../header";
import { RouteNames } from "../../routes/sources";
import { lazy } from "react";

const AboutContent = lazy(() => import("../../routes/about"));
const BrandBookContent = lazy(() => import("../../routes/brandbook"));

export function RightPanel() {
    return (
        <div className="right-panel">
            <Header />
            <Routes>
                <Route path="/">
                    <Route index element={<Navigate to={`/${RouteNames.BrandBook}`} replace />} />
                    <Route path={`/${RouteNames.BrandBook}`} element={<BrandBookContent />} />
                    <Route path={`/${RouteNames.CreateStand}`} element={<></>} />
                    <Route path={`/${RouteNames.About}`} element={<AboutContent />} />
                    <Route path={`/${RouteNames.Settings}`} element={<></>} />
                </Route>
            </Routes>
        </div>
    );
}

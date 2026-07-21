import { useState } from "react";
import { RouterProvider } from "./routes/router";
import { RightPanel } from "./widgets/right-panel";
import { Sider } from "./widgets/sidebar";
import "./scene.scss";

export default function Scene() {
    const [sidebarCollapsed, setSidebarCollapsed] = useState(false);

    const toggleSidebar = () => setSidebarCollapsed((v) => !v);

    return (
        <div className="scene">
            <RouterProvider>
                <Sider collapsed={sidebarCollapsed} />
                <RightPanel onToggleSidebar={toggleSidebar} sidebarCollapsed={sidebarCollapsed} />
            </RouterProvider>
        </div>
    );
}

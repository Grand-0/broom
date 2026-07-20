import { RouterProvider } from "./routes/router";
import { RightPanel } from "./widgets/right-panel";
import { Sider } from "./widgets/sidebar";

export default function Scene() {
    return (
        <div className="scene">
            <RouterProvider>
                <Sider />
                <RightPanel />
            </RouterProvider>
        </div>
    );
}

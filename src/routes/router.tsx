import { BrowserRouter, BrowserRouterProps } from "react-router-dom";

export function RouterProvider({ children }: Pick<BrowserRouterProps, "children">) {
    return <BrowserRouter children={children} />;
}

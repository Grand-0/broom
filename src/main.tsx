import React, { lazy } from "react";
import ReactDOM from "react-dom/client";
import "./styles/styles.scss";

const Scene = lazy(() => import("./scene"))

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Scene />
  </React.StrictMode>,
);

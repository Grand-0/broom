import { Router } from "../../../routes/router";
import { MasterList } from "../master-list";
import { BrowserRouter } from "react-router-dom";

export function Content() {
  return (
    <BrowserRouter>
      <div className="content">
        <MasterList />
        <Router />
      </div>
    </BrowserRouter>
  );
}

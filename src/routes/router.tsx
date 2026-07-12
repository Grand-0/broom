import { Route, Routes, useNavigate } from "react-router-dom";
import BrandBookScene from "./brandbook/scene";
import { useEffect } from "react";

export function Router() {
  const navigate = useNavigate();

  useEffect(() => {
    navigate("/Brandbook");
  }, []);

  return (
    <Routes>
      <Route path="/Brandbook" element={<BrandBookScene />} />
      <Route path="/LD" element={<>LD</>} />
    </Routes>
  );
}

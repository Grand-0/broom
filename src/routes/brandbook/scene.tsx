import { BrandCard } from "./brand-card";

export default function BrandBookScene() {
  return (
    <div className="content-scene">
      <BrandCard
        name="Win.dev.loc"
        os="windows"
        product={{
          dataBase: "postgree",
          name: "devalt9.loc",
          version: "20260702.28",
        }}
      />
    </div>
  );
}

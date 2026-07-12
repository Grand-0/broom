import { Content } from "./components/content/content";
import { Header } from "./components/header";

export default function Scene() {
  return (
    <main className="layout">
      <Header />
      <Content />
    </main>
  );
}

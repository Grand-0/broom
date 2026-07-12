import { NavigationCard } from "./navigation-card";

export function MasterList() {
  return (
    <nav className="navigation">
      <NavigationCard
        title="Brandbook"
        imagePath="/spellbook-magic.svg"
        toPath="/Brandbook"
      />
      <NavigationCard
        title="Settings"
        imagePath="/candlelabra-light.svg"
        toPath="/Settings"
      />
      <NavigationCard title="About" imagePath="/raven.svg" toPath="/About" />
    </nav>
  );
}

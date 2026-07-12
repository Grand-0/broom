import { matchPath, useLocation, useNavigate } from "react-router-dom";

type NavigationCardProps = {
  title: string;
  imagePath: string;
  toPath: string;
};

export function NavigationCard({
  title,
  imagePath,
  toPath,
}: NavigationCardProps) {
  const navigate = useNavigate();
  const location = useLocation();

  const isActive = matchPath({ path: toPath }, location.pathname) !== null;

  return (
    <div
      className={isActive ? "navigation-card active" : "navigation-card"}
      onClick={() => navigate(toPath)}
    >
      <img height={40} width={40} src={imagePath} />
      <p>{title}</p>
    </div>
  );
}

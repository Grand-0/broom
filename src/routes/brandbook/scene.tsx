import { StandCard } from "../../components/stand-card";
import { useStandsInfo } from "./hooks/use-get-stands-info";

export default function BrandBookScene() {
    const { result } = useStandsInfo();

    if (typeof result === "undefined") {
        return <></>;
    }

    if (result instanceof Error) {
        return <>{result.message}</>;
    }

    if (!result.length) {
        return <>Отсутствует информация о стендах</>;
    }

    return (
        <div className="content-scene">
            {result.map((r) => (
                <StandCard standInfo={r} />
            ))}
        </div>
    );
}

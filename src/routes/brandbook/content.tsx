import { StandCard } from "../../components/stand-card";
import { useStandsInfo } from "./hooks/use-get-stands-info";

import "./brandbook.scss";

export function BrandBookContent() {
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
        <div className="content">
            <div className="brandbook-header">
                <button className="add-stand-btn">
                    <p>+ Add Stand</p>
                </button>
            </div>
            <div className="brandbook-stand-list">
                {result.map((s) => (
                    <StandCard standInfo={s} />
                ))}
                <StandCard
                    standInfo={{
                        app_name: "TestData",
                        db_admin: "PostgreSQL",
                        db_owner: "root",
                        db_provider: "PostgreSQL",
                        db_server: "SERVER",
                        instance_dir: "opt/delo",
                        port_a: 10001,
                        port_b: 10002,
                        smb_server_address: "2113123",
                        target_account: "sdsad",
                        target_server_name: "dsfddsf",
                        web_server_name: "sdfsdfdsf",
                    }}
                />
            </div>
        </div>
    );
}

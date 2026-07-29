import { useState } from "react";
import { IStandInfo } from "../../models";

import "./stand-card.scss";
import { ArrowIcon } from "../icons";

type StandCardProps = {
    standInfo: IStandInfo;
    // v2: status: "running" | "stopped";
};

function DetailField({ label, value }: { label: string; value: string }) {
    return (
        <div className="detail-field">
            <span className="detail-label">{label}</span>
            <span className="detail-value">{value}</span>
        </div>
    );
}

export function StandCard({ standInfo }: StandCardProps) {
    const [collapsed, setCollapsed] = useState(true);

    return (
        <div className="stand-card">
            <div className="stand-card-header">
                <div className="stand-card-main-info">
                    {/* v2: status dot */}
                    {/* <div className={`status-dot ${isRunning ? "running" : "stopped"}`} /> */}
                    <span className="stand-card-app-name">{standInfo.webServerName}</span>
                </div>

                {/* v2: status badge */}
                {/* <div className={`status-badge ${isRunning ? "running" : "stopped"}`}>
                    {isRunning ? "Running" : "Stopped"}
                </div> */}

                <div className="stand-card-spacer" />

                <div className="stand-card-controls">
                    {/* v2: run / stop / refresh buttons */}
                    {/* <button className="control-btn success" title="Run">▶️</button> */}
                    {/* <button className="control-btn danger" title="Stop">⏹</button> */}
                    {/* <button className="control-btn ghost" title="Refresh">⟲</button> */}
                    <button className="control-btn destructive" title="Delete">
                        🗑
                    </button>
                    <button
                        className="control-btn ghost"
                        title="Collapse"
                        onClick={() => setCollapsed((c) => !c)}
                    >
                        <ArrowIcon className={`collapse-arrow ${collapsed ? "collapsed" : ""}`} />
                    </button>
                </div>
            </div>

            <div className={`stand-card-divider ${collapsed ? "hidden" : ""}`} />

            <div className={`stand-card-details ${collapsed ? "collapsed" : ""}`}>
                <div className="detail-row">
                    <DetailField label="Server" value={standInfo.targetServerName} />
                    <DetailField label="Account" value={standInfo.targetAccount} />
                    <DetailField label="Directory" value={standInfo.instanceDir} />
                </div>
                <div className="detail-row">
                    <DetailField label="Instance" value={standInfo.appName} />
                    <DetailField label="Main Port" value={String(standInfo.portA)} />
                    <DetailField label="Secondary Port" value={String(standInfo.portB)} />
                </div>
                <div className="detail-row">
                    <DetailField label="DB Provider" value={standInfo.dbProvider} />
                    <DetailField label="DB Server" value={standInfo.dbServer} />
                    <DetailField label="SMB" value={standInfo.smbServerAddress} />
                </div>
            </div>
        </div>
    );
}

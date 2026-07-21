export function AboutContent() {
    return (
        <div className="content">
            <div className="about-card">
                <img src="/icons/broom.svg" />
                <h1>Broom</h1>
                <h5>Version {import.meta.env["APP_VERSION"]}</h5>
                <h6>Desktop application for managing Hyper-V stands</h6>
                <div className="devider"></div>
                <div className="tech-info">
                    <p>Built with Tauri v2 + React 19 + Rust</p>
                    <p>Powershell backend for Hyper-V management</p>
                </div>
            </div>
        </div>
    );
}

import "./radio-button.scss";

type RadioOption = {
    label: string;
    value: string;
};

type RadioGroupProps = {
    label: string;
    options: RadioOption[];
    value?: string;
    onChange?: (value: string) => void;
};

export function RadioGroup({ label, options, value, onChange }: RadioGroupProps) {
    return (
        <div className="radio-group">
            <span className="radio-label">{label}</span>
            <div className="radio-container">
                {options.map((opt) => {
                    const selected = opt.value === value;
                    return (
                        <button
                            key={opt.value}
                            className="radio-option"
                            onClick={() => onChange?.(opt.value)}
                            type="button"
                        >
                            <span className={`radio-circle${selected ? " selected" : ""}`}>
                                {selected && <span className="radio-dot" />}
                            </span>
                            <span className={`radio-label-text${selected ? "" : " muted"}`}>
                                {opt.label}
                            </span>
                        </button>
                    );
                })}
            </div>
        </div>
    );
}

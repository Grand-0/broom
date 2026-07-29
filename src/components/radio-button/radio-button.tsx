import "./radio-button.scss";

export type RadioOption<T> = {
    label: string;
    value: T;
};

type RadioGroupProps<T> = {
    label: string;
    options: RadioOption<T>[];
    value?: T;
    onChange?: (value: T) => void;
};

export function RadioGroup<T>({ label, options, value, onChange }: RadioGroupProps<T>) {
    return (
        <div className="radio-group">
            <span className="radio-label">{label}</span>
            <div className="radio-container">
                {options.map((opt) => {
                    const selected = opt.value === value;
                    return (
                        <button
                            key={`${opt.value}`}
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

import "./checkbox.scss";

type CheckboxProps = {
    label: string;
    checked?: boolean;
    onChange?: (checked: boolean) => void;
};

export function Checkbox({ label, checked = false, onChange }: CheckboxProps) {
    return (
        <button
            className="checkbox"
            onClick={() => onChange?.(!checked)}
            type="button"
        >
            <span className={`checkbox-box${checked ? " checked" : ""}`}>
                {checked && <span className="checkbox-mark">✓</span>}
            </span>
            <span className="checkbox-label">{label}</span>
        </button>
    );
}

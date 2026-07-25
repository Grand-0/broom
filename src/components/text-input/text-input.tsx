import "./text-input.scss";

type TextInputProps = {
    label: string;
    value?: string;
    onChange?: (value: string) => void;
    placeholder?: string;
};

export function TextInput({ label, value = "", onChange, placeholder }: TextInputProps) {
    return (
        <div className="text-input">
            <span className="text-input-label">{label}</span>
            <input
                className="text-input-field"
                type="text"
                value={value}
                onChange={(e) => onChange?.(e.target.value)}
                placeholder={placeholder}
            />
        </div>
    );
}

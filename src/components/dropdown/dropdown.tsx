import { useState, useCallback, useRef, useEffect } from "react";

import "./dropdown.scss";

export type DropdownOption = {
    label: string;
    value: string;
};

type DropdownProps = {
    label: string;
    options: DropdownOption[];
    value?: string;
    onChange?: (value: string) => void;
    placeholder?: string;
};

export function Dropdown({ label, options, value, onChange, placeholder }: DropdownProps) {
    const [open, setOpen] = useState(false);
    const ref = useRef<HTMLDivElement>(null);

    const selected = options.find((o) => o.value === value);
    const toggle = useCallback(() => setOpen((v) => !v), []);
    const select = useCallback(
        (v: string) => {
            onChange?.(v);
            setOpen(false);
        },
        [onChange]
    );

    useEffect(() => {
        const handler = (e: MouseEvent) => {
            if (ref.current && !ref.current.contains(e.target as Node)) {
                setOpen(false);
            }
        };
        document.addEventListener("mousedown", handler);
        return () => document.removeEventListener("mousedown", handler);
    }, []);

    return (
        <div className="dropdown" ref={ref}>
            <span className="dropdown-label">{label}</span>
            <button className="dropdown-trigger" onClick={toggle} type="button">
                <span className={`dropdown-value${selected ? "" : " placeholder"}`}>
                    {selected ? selected.label : (placeholder ?? "Select...")}
                </span>
                <span className={`dropdown-arrow ${open ? "open" : ""}`}>▲</span>
            </button>
            {open && (
                <div className="dropdown-menu">
                    {options.map((opt) => (
                        <button
                            key={opt.value}
                            className={`dropdown-option${opt.value === value ? " selected" : ""}`}
                            onClick={() => select(opt.value)}
                            type="button"
                        >
                            <span className="dropdown-option-text">{opt.label}</span>
                            {opt.value === value && (
                                <span className="dropdown-option-check">✓</span>
                            )}
                        </button>
                    ))}
                </div>
            )}
        </div>
    );
}

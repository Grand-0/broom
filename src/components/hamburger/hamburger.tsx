import { CloseIcon, HamburgerIcon } from "../icons";
import "./hamburger.scss";

type HamburgerProps = {
    active?: boolean;
    onClick?: () => void;
};

export function Hamburger({ active = false, onClick }: HamburgerProps) {
    return (
        <button
            className={`hamburger-btn${active ? " active" : ""}`}
            onClick={onClick}
            type="button"
        >
            {active ? <CloseIcon /> : <HamburgerIcon />}
        </button>
    );
}

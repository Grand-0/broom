type HeaderProps = {
    title: string;
};

export function Header({ title }: HeaderProps) {
    return (
        <div className="header">
            <div className="hamburger"></div>
            <h4>{title}</h4>
        </div>
    );
}

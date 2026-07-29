import { DropdownOption } from "../../../components/dropdown";

export function getDropdownOptions(values: string[]): DropdownOption[] {
    return values.map((v) => ({ label: v, value: v }));
}

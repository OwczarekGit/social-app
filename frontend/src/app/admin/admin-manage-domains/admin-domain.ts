import {ListDisplay} from "../../data/list-display";

export class AdminDomain implements ListDisplay {
  value: string

  constructor(value: string) {
    this.value = value;
  }

  display(): string {
    return this.value;
  }

  iconUrl(): string {
    return "/assets/manage-domain-s.png";
  }
}

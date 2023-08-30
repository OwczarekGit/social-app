import {ListDisplay} from "./list-display";

export class Tag implements ListDisplay {
  name: string


  constructor(name: string) {
    this.name = name;
  }

  display(): string {
    return this.name
  }

  iconUrl(): string {
    return ""
  }
}

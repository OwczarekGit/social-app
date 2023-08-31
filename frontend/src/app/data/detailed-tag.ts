import {ListDisplay} from "./list-display";

export class DetailedTag implements ListDisplay {
  id: number
  name: string
  usage: number


  constructor(id: number, name: string, usage: number) {
    this.id = id;
    this.name = name;
    this.usage = usage;
  }

  display(): string {
    return `(${this.usage}) ${this.name}`
  }

  iconUrl(): string {
    return "/assets/tag-s.png"
  }
}

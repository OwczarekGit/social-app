import {ListDisplay} from "./list-display";

export class Profile implements ListDisplay {
  id: number
  username: string

  constructor(id: number, username: string) {
    this.id = id
    this.username = username
  }

  display(): string {
    return this.username
  }

  iconUrl(): string {
    return "/assets/user-icon-s.png"
  }
}

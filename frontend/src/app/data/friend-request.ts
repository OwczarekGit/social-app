import {ListDisplay} from "./list-display";

export class FriendRequest implements ListDisplay {

  user_id: number
  username: string

  constructor(id: number, username: string) {
    this.user_id = id;
    this.username = username;
  }

  display(): string {
    return this.username
  }
}

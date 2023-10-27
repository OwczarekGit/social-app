import {ListDisplay} from "./list-display";

export class Profile implements ListDisplay {
  user_id: number
  username: string
  picture_url: string

  constructor(user_id: number, username: string, picture_url: string) {
    this.user_id = user_id;
    this.username = username;
    this.picture_url = picture_url;
  }

  display(): string {
    return this.username
  }

  iconUrl(): string {
    return "/assets/user-icon-s.png"
  }
}

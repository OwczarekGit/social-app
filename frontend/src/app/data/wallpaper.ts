import {ListDisplay} from "./list-display";

export class Wallpaper implements ListDisplay {
  id: number
  title: string
  url: string

  constructor(id: number, title: string, url: string) {
    this.id = id;
    this.title = title;
    this.url = url
  }

  display(): string {
    return this.title;
  }

  iconUrl(): string {
    return "/assets/wallpaper-settings-s.png";
  }
}

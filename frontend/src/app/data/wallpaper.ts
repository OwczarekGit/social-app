import {ListDisplay} from "./list-display";
export class Wallpaper implements ListDisplay {
  id: number
  title: string
  url: string

  constructor(id: number, title: string, url: string) {
    this.id = id;
    this.title = title;
    this.url = `http://192.168.1.45:9000${url}`;
  }

  display(): string {
    return this.title;
  }

  iconUrl(): string {
    return "/assets/wallpaper-settings-s.png";
  }
}

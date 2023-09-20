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
    let url = ''

    switch(this.name) {
      case 'Wallpaper': url = 'wallpaper-settings-s.png'; break;
      case 'NSFW': url = 'error.png'; break;
      default: url = 'tag-s.png'; break;
    }

    return '/assets/' + url
  }
}

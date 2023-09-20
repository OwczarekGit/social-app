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
    let url = ''

    switch(this.name) {
      case 'Wallpaper': url = 'wallpaper-settings-s.png'; break;
      case 'NSFW': url = 'error.png'; break;
      default: url = 'tag-s.png'; break;
    }

    return '/assets/' + url
  }
}

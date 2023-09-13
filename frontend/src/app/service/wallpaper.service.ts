import {inject, Injectable} from '@angular/core';
import {WindowService} from "./window.service";

@Injectable({
  providedIn: 'root'
})
export class WallpaperService {

  public windowService = inject(WindowService)

  constructor() {}

  public setWallpaper(url: string) {
    let el = (this.windowService.vcr?.element.nativeElement as HTMLDivElement)
    el.style.background = `url(${url}) no-repeat center center`;
    el.style.backgroundSize = 'cover'
    localStorage.setItem("wallpaper", url)
  }

  public restoreWallpaper() {
    let wall = localStorage.getItem("wallpaper")
    if (wall != null) {
      this.setWallpaper(wall)
    }
  }

  public resetWallpaper() {
    let el = (this.windowService.vcr?.element.nativeElement as HTMLDivElement)
    el.style.background = ``
    localStorage.removeItem("wallpaper")
  }
}

import {inject, Injectable} from '@angular/core';
import {WindowService} from "./window.service";
import {Observable} from "rxjs";
import {Wallpaper} from "../data/wallpaper";
import {HttpClient} from "@angular/common/http";

@Injectable({
  providedIn: 'root'
})
export class WallpaperService {

  public windowService = inject(WindowService)
  public http = inject(HttpClient)

  constructor() {}

  public setWallpaper(id: number): Observable<Wallpaper> {
    return this.http.post<Wallpaper>("/api/wallpaper/" + id, {})
  }

  public restoreWallpaper() {
    this.http.get<Wallpaper | null>("/api/wallpaper/current").subscribe({
      next: v => {
        let el = (this.windowService.vcr?.element.nativeElement as HTMLDivElement)
        if (v != null) {
          let value = new Wallpaper(v.id, v.title, v.url);
          el.style.backgroundImage = `url(${value.url})`
        } else {
          el.style.backgroundImage = ''
        }
      }
    })
  }

  public resetWallpaper() {
    this.http.delete("/api/wallpaper").subscribe({
      next: value => {
        this.restoreWallpaper()
      }
    })
  }

  public getAllWallpapers(): Observable<Wallpaper[]> {
    return this.http.get<Wallpaper[]>("/api/wallpaper")
  }
}

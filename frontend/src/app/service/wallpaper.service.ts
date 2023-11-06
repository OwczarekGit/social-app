import {inject, Injectable} from '@angular/core';
import {Observable} from "rxjs";
import {Wallpaper} from "../data/wallpaper";
import {HttpClient} from "@angular/common/http";
import {Maybe, None} from "option-value";

@Injectable({
  providedIn: 'root'
})
export class WallpaperService {

  public http = inject(HttpClient)
  public backgroundElement = None<HTMLDivElement>()

  constructor() {}

  public setWallpaper(id: number): Observable<Wallpaper> {
    return this.http.post<Wallpaper>("/api/wallpaper/" + id, {})
  }

  public restoreWallpaper() {
    this.http.get<Wallpaper | null>("/api/wallpaper/current").subscribe({
      next: v => {
        this.backgroundElement
          .ifPresent(el => {
            Maybe(v)
              .ifPresentOrElse(wallpaper => {
                let wall = new Wallpaper(wallpaper.id, wallpaper.title, wallpaper.url)
                el.style.backgroundImage = `url(${wall.url})`
              }, () => this.setDefaultBackground())
          })
      }
    })
  }

  public resetWallpaper() {
    this.http.delete("/api/wallpaper").subscribe({
      next: _ => {
        this.restoreWallpaper()
      }
    })
  }

  public getAllWallpapers(): Observable<Wallpaper[]> {
    return this.http.get<Wallpaper[]>("/api/wallpaper")
  }

  public setDefaultBackground() {
    this.backgroundElement.ifPresent(el => {
      el.style.backgroundImage = ''
    })
  }
}

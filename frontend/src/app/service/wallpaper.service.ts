import {inject, Injectable} from '@angular/core';
import {WindowService} from "./window.service";
import {Observable} from "rxjs";
import {Wallpaper} from "../data/wallpaper";
import {HttpClient} from "@angular/common/http";
import {DomainService} from "./domain.service";
import {Maybe} from "option-value";

@Injectable({
  providedIn: 'root'
})
export class WallpaperService {

  public windowService = inject(WindowService)
  public domainService = inject(DomainService)
  public http = inject(HttpClient)

  constructor() {}

  public setWallpaper(id: number): Observable<Wallpaper> {
    return this.http.post<Wallpaper>("/api/wallpaper/" + id, {})
  }

  public restoreWallpaper() {
    this.http.get<Wallpaper | null>("/api/wallpaper/current").subscribe({
      next: v => {
        this.windowService.vcr
          .ifPresent(vcr => {
            let el = (vcr.element.nativeElement as HTMLDivElement)
            Maybe(v)
              .ifPresentOrElse(wallpaper => {
                let wall = new Wallpaper(wallpaper.id, wallpaper.title, wallpaper.url)
                el.style.backgroundImage = `url(${wall.url})`
              }, () => el.style.backgroundImage = ``)
          })
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

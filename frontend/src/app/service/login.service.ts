import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {WallpaperService} from "./wallpaper.service";

@Injectable({
  providedIn: 'root'
})
export class LoginService {

  private http = inject(HttpClient)
  private wallpaperService = inject(WallpaperService)

  constructor() { }

  public login(email: string, password: string): Observable<any> {
    return this.http.post("/api/account/login", {
      email: email,
      password: password
    })
  }

  public logout(): Observable<any> {
    this.wallpaperService.setDefaultBackground()
    return this.http.delete("/api/account/logout")
  }
}

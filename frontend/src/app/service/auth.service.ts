import {inject, Injectable} from '@angular/core';
import {CookieService} from "ngx-cookie-service";

@Injectable({
  providedIn: 'root'
})
export class AuthService {

  cookieService = inject(CookieService)

  constructor() { }

  public isLoggedIn(): boolean {
    return this.cookieService.check("AUTH")
  }

  public isNotLoggedIn(): boolean {
    return !this.cookieService.check("AUTH")
  }
}

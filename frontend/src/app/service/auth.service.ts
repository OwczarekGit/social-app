import {inject, Injectable} from '@angular/core';
import {CookieService} from "ngx-cookie-service";
import {AccountType} from "../const/account-type";

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

  public getAccountType(): AccountType {
    return this.cookieService.get("ROLE") as AccountType
  }

  public isAdmin(): boolean {
    return this.getAccountType() == AccountType.Admin
  }
}

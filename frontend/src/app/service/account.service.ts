import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class AccountService {

  private http = inject(HttpClient)

  constructor() { }

  public changePassword(oldPassword: string, newPassword: string): Observable<any> {
    return this.http.put("/api/account/password", {
      old_password: oldPassword,
      new_password: newPassword,
    })
  }
}

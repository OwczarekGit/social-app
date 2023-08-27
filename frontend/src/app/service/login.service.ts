import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class LoginService {

  private http = inject(HttpClient)

  constructor() { }

  public login(email: string, password: string): Observable<any> {
    return this.http.post("/api/account/login", {
      email: email,
      password: password
    })
  }

  public logout(): Observable<any> {
    return this.http.delete("/api/account/logout")
  }
}

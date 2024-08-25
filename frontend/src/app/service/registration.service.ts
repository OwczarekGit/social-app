import {inject, Injectable} from '@angular/core';
import { HttpClient } from "@angular/common/http";
import {Observable} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class RegistrationService {

  private http = inject(HttpClient)

  constructor() { }

  public register(username: string, email: string, password: string): Observable<any> {
    return this.http.post("/api/account", {
      username: username,
      email: email,
      password: password
    })
  }
}

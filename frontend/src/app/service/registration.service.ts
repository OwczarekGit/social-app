import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {map, Observable} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class RegistrationService {

  private http = inject(HttpClient)

  constructor() { }

  public register(email: string, password: string): Observable<any> {
    return this.http.post("/api/account", {
      email: email,
      password: password
    })
  }
}

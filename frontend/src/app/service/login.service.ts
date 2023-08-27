import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";

@Injectable({
  providedIn: 'root'
})
export class LoginService {

  private http = inject(HttpClient)

  constructor() { }

  public login(email: string, password: string) {}
}

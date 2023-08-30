import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {Profile} from "../data/profile";

@Injectable({
  providedIn: 'root'
})
export class ProfileService {

  private http = inject(HttpClient)

  constructor() { }

  changeUsername(username: string): Observable<any> {
    return this.http.put("/api/profile/username", {
      username: username
    })
  }

  getMyProfile(): Observable<Profile> {
    return this.http.get<Profile>("/api/profile/username")
  }
}

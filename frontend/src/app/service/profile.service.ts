import {inject, Injectable} from '@angular/core';
import { HttpClient } from "@angular/common/http";
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
    return this.http.get<Profile>("/api/profile")
  }

  getProfileForUserId(params: number): Observable<Profile> {
    return this.http.get<Profile>("/api/profile/" + params)
  }

  public setProfilePicture(file: File): Observable<any> {
    let form = new FormData()
    form.set("image", file)
    return this.http.put("/api/profile/picture", form)
  }
}

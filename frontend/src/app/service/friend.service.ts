import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {FriendRequest} from "../data/friend-request";

@Injectable({
  providedIn: 'root'
})
export class FriendService {

  private http = inject(HttpClient)

  constructor() { }

  public getPendingFriendRequests(): Observable<FriendRequest[]> {
    return this.http.get<FriendRequest[]>("/api/friend/request/pending")
  }

  public acceptFriendRequest(requesterId: number): Observable<any> {
    return this.http.post("/api/friend/request/accept/"+ requesterId,{})
  }

  denyFriendRequest(requesterId: number): Observable<any> {
    return this.http.post("/api/friend/request/accept/"+ requesterId,{})
  }
}

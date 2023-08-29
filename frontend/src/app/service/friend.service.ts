import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {FriendRequest} from "../data/friend-request";
import {SearchNonFriendResult} from "../data/search-non-friend-result";

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

  public denyFriendRequest(requesterId: number): Observable<any> {
    return this.http.post("/api/friend/request/deny/"+ requesterId,{})
  }

  public searchNonFriends(phrase: string): Observable<SearchNonFriendResult[]> {
    return this.http.get<SearchNonFriendResult[]>("/api/friend?phrase=" + phrase)
  }

  public sendFriendRequest(id: number): Observable<any> {
    return this.http.post("/api/friend/invite/" + id, {})
  }
}

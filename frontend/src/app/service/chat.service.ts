import {inject, Injectable} from '@angular/core';
import { HttpClient } from "@angular/common/http";
import {Observable} from "rxjs";
import {FriendMessage} from "../data/friend-message";

@Injectable({
  providedIn: 'root'
})
export class ChatService {

  private http = inject(HttpClient)

  constructor() { }

  public sendMessageToFriend(friendId: number, message: string): Observable<FriendMessage> {
    return this.http.post<FriendMessage>("/api/chat/friend", {
      friend_id: friendId,
      message: message
    })
  }

  public getMessagesForFriendConversation(friendId: number): Observable<FriendMessage[]> {
    return this.http.get<FriendMessage[]>("/api/chat/friend/" + friendId)
  }
}

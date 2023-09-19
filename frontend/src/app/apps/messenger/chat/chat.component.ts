import {Component, inject, Input, signal} from '@angular/core';
import {Profile} from "../../../data/profile";
import {ChatService} from "../../../service/chat.service";
import {FriendMessage} from "../../../data/friend-message";
import {NotificationService} from "../../../service/notification.service";
import {NotificationType} from "../../../const/notification-type";
import {single} from "rxjs";

@Component({
  selector: 'app-chat',
  templateUrl: './chat.component.html',
  styleUrls: ['./chat.component.css']
})
export class ChatComponent {
  public chatService = inject(ChatService)
  public notificationService = inject(NotificationService)

  @Input()
  set myProfile(value: Profile) {
    this._myProfile.set(value)
  }

  public _myProfile = signal<Profile | null>(null)

  _profile = signal<Profile | null>(null)

  public messages = signal<FriendMessage[]>([])

  public messageText: string = ''

  constructor() {
    this.notificationService.onNewNotification.subscribe(v => {
      if (v.notification_data.notification_type == NotificationType.MESSAGE) {
        this.messages.update(o => {
          // @ts-ignore
          let n: FriendMessage = v.notification_data.data as FriendMessage
          return [n, ...o]
        })
      }
    })
  }

  @Input('profile')
  set profile(value: Profile) {
    this._profile.set(value)
    if (this._profile()?.user_id != null) {
      this.chatService.getMessagesForFriendConversation(this._profile()?.user_id as number).subscribe({
        next: v => {
          this.messages.set(v)
        }
      })
    }
  }

  sendMessage() {
    let id = this._profile()?.user_id
    if (this.messageText == '' && id != null) return

    this.chatService.sendMessageToFriend(id as number, this.messageText).subscribe({
      next: value => this.messages.update(o => [value, ...o])
    })


    this.messageText = ''
  }
}

import {AfterViewInit, Component, HostBinding, Input, OnInit} from '@angular/core';
import {FriendMessage} from "../../../../data/friend-message";

@Component({
  selector: 'app-chat-friend-message',
  templateUrl: './chat-friend-message.component.html',
  styleUrls: ['./chat-friend-message.component.css']
})
export class ChatFriendMessageComponent {
  @Input()
  message?: FriendMessage

  @Input()
  isMine: boolean = false
}

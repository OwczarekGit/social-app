import {Component, Input} from '@angular/core';
import {Profile} from "../../../data/profile";

@Component({
  selector: 'app-chat',
  templateUrl: './chat.component.html',
  styleUrls: ['./chat.component.css']
})
export class ChatComponent {
  @Input('profile')
  profile?: Profile

  public messageText: string = ''

  sendMessage() {
    if (this.messageText == '') return



    this.messageText = ''
  }
}

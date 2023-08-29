import {Component, EventEmitter, Input, Output} from '@angular/core';
import {Notification} from "../../../data/notification";
import {NotificationType} from "../../../const/notification-type";

@Component({
  selector: 'app-notification-entry',
  templateUrl: './notification-entry.component.html',
  styleUrls: ['./notification-entry.component.css']
})
export class NotificationEntryComponent {
  @Input()
  notification!: Notification

  @Output()
  dismissClicked: EventEmitter<Notification> = new EventEmitter<Notification>()

  public getIcon(): string {
    switch (this.notification.notification_data.notification_type) {
      case NotificationType.FRIEND_REQUEST: return `/assets/friends.png`;
      case NotificationType.MESSAGE: return `/assets/friends.png`;
    }
  }

  public getContent(): string {
    switch (this.notification.notification_data.notification_type) {
      case NotificationType.FRIEND_REQUEST: {
        return "You have a new friend request."
      }
      case NotificationType.MESSAGE: {
        return "New message."
      }
    }
  }
}

import {AfterViewInit, Component, inject, ViewChild} from '@angular/core';
import {WindowComponent} from "../../ui-elements/window/window.component";
import {NotificationService} from "../../service/notification.service";
import {Notification} from "../../data/notification";

@Component({
  selector: 'app-notification-center',
  templateUrl: './notification-center.component.html',
  styleUrls: ['./notification-center.component.css']
})
export class NotificationCenterComponent {
  @ViewChild(WindowComponent)
  window!: WindowComponent

  public notificationService = inject(NotificationService)

  public close() {
    this.window.closeWindow()
  }

  dismiss(notification: Notification) {
    this.notificationService.dismissNotification(notification.id)
  }
}

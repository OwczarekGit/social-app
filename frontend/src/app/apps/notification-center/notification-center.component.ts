import {AfterViewInit, Component, inject} from '@angular/core';
import {NotificationService} from "../../service/notification.service";
import {Notification} from "../../data/notification";
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";

@Component({
    selector: 'app-notification-center',
    templateUrl: './notification-center.component.html',
    styleUrls: ['./notification-center.component.css'],
    standalone: false
})
export class NotificationCenterComponent extends WindowContent<null, W2kWindowFrameComponent> implements AfterViewInit {

  public notificationService = inject(NotificationService)

  public close() {
    this.closeWindow()
  }

  dismiss(notification: Notification) {
    this.notificationService.dismissNotification(notification.id)
  }

  ngAfterViewInit(): void {
    this.windowFrame.onClose = () => this.closeWindow()
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)
    setTimeout(() => {
      this.setTitle("Notifications")
      this.setIcon("/assets/notification-icon.png")
    })
  }
}

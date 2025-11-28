import {Component, inject, Input} from '@angular/core';
import {NotificationCenterComponent} from "../../../../apps/notification-center/notification-center.component";
import {WindowService} from "../../../../service/window.service";
import {W2kWindowFrameComponent} from "../../../../ui-elements/w2k-window-frame/w2k-window-frame.component";

@Component({
    selector: 'app-notification-indicator',
    templateUrl: './notification-indicator.component.html',
    styleUrls: ['./notification-indicator.component.css'],
    host: { '(click)': 'openNotificationCenter()' },
    standalone: false
})
export class NotificationIndicatorComponent {
  @Input()
  notificationCount: number = 0

  private windowService = inject(WindowService)

  openNotificationCenter() {
    this.windowService.openApplication(NotificationCenterComponent, null ,W2kWindowFrameComponent)
  }
}

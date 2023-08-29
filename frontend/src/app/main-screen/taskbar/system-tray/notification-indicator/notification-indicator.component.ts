import {Component, inject, Input} from '@angular/core';
import {WindowService} from "../../../../service/window.service";
import {NotificationCenterComponent} from "../../../../apps/notification-center/notification-center.component";

@Component({
  selector: 'app-notification-indicator',
  templateUrl: './notification-indicator.component.html',
  styleUrls: ['./notification-indicator.component.css'],
  host: {'(click)': 'openNotificationCenter()'}
})
export class NotificationIndicatorComponent {
  @Input()
  notificationCount: number = 0

  private windowService = inject(WindowService)

  openNotificationCenter() {
    this.windowService.openApplication(NotificationCenterComponent)
  }
}

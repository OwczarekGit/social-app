import {Component, inject} from '@angular/core';
import {NotificationService} from "../../service/notification.service";

@Component({
  selector: 'app-taskbar',
  templateUrl: './taskbar.component.html',
  styleUrls: ['./taskbar.component.css']
})
export class TaskbarComponent {
  public notificationService = inject(NotificationService)
}

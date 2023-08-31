import {Component, inject} from '@angular/core';
import {NotificationService} from "../../service/notification.service";
import {WindowService} from "../../service/window.service";
import {PostWriterComponent} from "../../apps/post-writer/post-writer.component";
import {ShareImageComponent} from "../../apps/share-image/share-image.component";

@Component({
  selector: 'app-taskbar',
  templateUrl: './taskbar.component.html',
  styleUrls: ['./taskbar.component.css']
})
export class TaskbarComponent {
  public notificationService = inject(NotificationService)
  public windowService = inject(WindowService)

  public openWritePost() {
    this.windowService.openApplication(PostWriterComponent)
  }

  public openShareImage() {
    this.windowService.openApplication(ShareImageComponent)
  }
}

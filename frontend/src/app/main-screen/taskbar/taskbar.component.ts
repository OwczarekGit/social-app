import {Component, inject} from '@angular/core';
import {NotificationService} from "../../service/notification.service";
import {PostWriterComponent} from "../../apps/post-writer/post-writer.component";
import {ShareImageComponent} from "../../apps/share-image/share-image.component";
import {WindowService} from "../../service/window.service";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";

@Component({
  selector: 'app-taskbar',
  templateUrl: './taskbar.component.html',
  styleUrls: ['./taskbar.component.css']
})
export class TaskbarComponent {
  public notificationService = inject(NotificationService)
  public windowService = inject(WindowService)

  public openWritePost() {
    this.windowService.openApplication(PostWriterComponent, null, W2kWindowFrameComponent)
  }

  public openShareImage() {
    this.windowService.openApplication(ShareImageComponent, null, W2kWindowFrameComponent)
  }
}

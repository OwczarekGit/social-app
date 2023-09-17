import {AfterViewInit, Component, inject, ViewChild, ViewContainerRef} from '@angular/core';
import {NotificationService} from "../service/notification.service";
import {WindowService} from "../service/window.service";
import {WallpaperService} from "../service/wallpaper.service";
import {
  AdminActivationEmailEditorComponent
} from "../admin/admin-activation-email-editor/admin-activation-email-editor.component";
import {W2kWindowFrameComponent} from "../ui-elements/w2k-window-frame/w2k-window-frame.component";

@Component({
  selector: 'app-main-screen',
  templateUrl: './main-screen.component.html',
  styleUrls: ['./main-screen.component.css']
})
export class MainScreenComponent implements AfterViewInit {

  @ViewChild('surface', {read: ViewContainerRef})
  surface!: ViewContainerRef

  private nWindowService = inject(WindowService)
  private notificationService = inject(NotificationService)
  private wallpaperService = inject(WallpaperService)

  ngAfterViewInit(): void {
    this.nWindowService.setDisplay(this.surface)
    this.notificationService.subscribeToNotifications()
    this.notificationService.loadRemainingNotifications()

    setTimeout(() => this.wallpaperService.restoreWallpaper())
  }

}

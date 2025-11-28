import {AfterViewInit, Component, inject, ViewChild, ViewContainerRef} from '@angular/core';
import {NotificationService} from "../service/notification.service";
import {WindowService} from "../service/window.service";
import {WallpaperService} from "../service/wallpaper.service";

@Component({
    selector: 'app-main-screen',
    templateUrl: './main-screen.component.html',
    styleUrls: ['./main-screen.component.css'],
    standalone: false
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

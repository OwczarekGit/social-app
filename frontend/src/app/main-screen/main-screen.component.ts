import {AfterViewInit, Component, ElementRef, inject, ViewChild, ViewContainerRef} from '@angular/core';
import {WindowService} from "../service/window.service";
import {NotificationService} from "../service/notification.service";

@Component({
  selector: 'app-main-screen',
  templateUrl: './main-screen.component.html',
  styleUrls: ['./main-screen.component.css']
})
export class MainScreenComponent implements AfterViewInit {

  @ViewChild('surface', {read: ViewContainerRef})
  surface!: ViewContainerRef

  private windowService = inject(WindowService)
  private notificationService = inject(NotificationService)

  ngAfterViewInit(): void {
    this.windowService.setSurface(this.surface)
    this.notificationService.subscribeToNotifications()
    this.notificationService.loadRemainingNotifications()
  }

}

import {ComponentRef, inject, Injectable} from '@angular/core';
import {WindowService} from "./window.service";
import {NotificationComponent} from "../apps/notification/notification.component";
import {SoundService} from "./sound.service";

@Injectable({
  providedIn: 'root'
})
export class NotificationService {

  private windowService = inject(WindowService)
  private soundService = inject(SoundService)

  constructor() { }

  public error(title: string, text: string) {
    let win = this.windowService.openApplication(NotificationComponent) as ComponentRef<NotificationComponent>
    win.instance.title = title
    win.instance.text = text
    win.instance.icon = "/assets/error.png"
    let [w,h] = this.windowService.getSurfaceSize()
    setTimeout(() => this.windowService.setPosition(win.instance.window.id, w/2, h/2))
    this.soundService.error()
  }

  info(title: string, text: string) {
    let win = this.windowService.openApplication(NotificationComponent) as ComponentRef<NotificationComponent>
    win.instance.title = title
    win.instance.text = text
    win.instance.icon = "/assets/info.png"
    let [w,h] = this.windowService.getSurfaceSize()
    setTimeout(() => this.windowService.setPosition(win.instance.window.id, w/2, h/2))
    this.soundService.message()
  }
}

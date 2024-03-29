import {inject} from "@angular/core";
import {WindowService} from "../service/window.service";
import {WindowFrame} from "./window-frame";

export abstract class WindowContent<P, F extends WindowFrame> {

  wm = inject(WindowService)
  windowFrame!: F

  id: number = -1
  title: string = ''
  iconUrl: string = ''

  public setParams(params: P): void {}

  setTitle(name: string) {
    this.title = name
    this.windowFrame.title.set(this.title)
  }

  setIcon(url: string) {
    this.iconUrl = url
    this.windowFrame.iconUrl.set(this.iconUrl)
  }

  closeWindow() {
    this.wm.closeApplication(this.id)
  }
}

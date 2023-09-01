import {inject, Injectable} from '@angular/core';
import {PopupComponent} from "../apps/popup/popup.component";
import {SoundService} from "./sound.service";
import {WindowService} from "./window.service";
import {PopupParams, PopupType} from "../apps/popup/popup-params";
import {W2kWindowFrameComponent} from "../ui-elements/w2k-window-frame/w2k-window-frame.component";

@Injectable({
  providedIn: 'root'
})
export class PopupService {

  private windowService = inject(WindowService)
  private soundService = inject(SoundService)

  constructor() { }

  public error(title: string, text: string) {
    let id = this.windowService.openApplication(PopupComponent, new PopupParams(PopupType.Error, text, title), W2kWindowFrameComponent)
    if (id != null) {
      this.centerPopup(id)
      this.soundService.error()
    }
  }

  public info(title: string, text: string) {
    let id = this.windowService.openApplication(PopupComponent, new PopupParams(PopupType.Info, text, title), W2kWindowFrameComponent)
    if (id != null) {
      this.centerPopup(id)
      this.soundService.message()
    }
  }

  private centerPopup(id: number) {
    setTimeout(() => {
      let [x,y] = this.windowService.getDisplaySize()
      this.windowService.setPosition(id, x/2, y/2, true)
    })
  }
}

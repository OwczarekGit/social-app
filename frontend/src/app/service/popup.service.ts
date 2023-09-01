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
    this.windowService.openApplication(PopupComponent, new PopupParams(PopupType.Error, text, title), W2kWindowFrameComponent)
    this.soundService.error()
  }

  info(title: string, text: string) {
    this.windowService.openApplication(PopupComponent, new PopupParams(PopupType.Info, text, title), W2kWindowFrameComponent)
    this.soundService.message()
  }
}

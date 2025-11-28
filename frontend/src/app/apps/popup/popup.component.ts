import {AfterViewInit, Component} from '@angular/core';
import {WindowContent} from "../../data/window-content";
import {PopupParams, PopupType} from "./popup-params";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";

@Component({
    selector: 'app-popup',
    templateUrl: './popup.component.html',
    styleUrls: ['./popup.component.css'],
    standalone: false
})
export class PopupComponent extends WindowContent<PopupParams, W2kWindowFrameComponent> implements AfterViewInit {

  public text!: string

  public close() {
    this.closeWindow()
  }

  override setParams(params: PopupParams) {
    this.text = params.text
    this.setTitle(params.title)
    switch (params.type) {
      case PopupType.Warning: this.setIcon("/assets/notification-icon.png"); break;
      case PopupType.Info: this.setIcon("/assets/info.png"); break;
      case PopupType.Error: this.setIcon("/assets/error.png"); break;
    }
  }

  ngAfterViewInit(): void {
    this.windowFrame.onClose = () => this.closeWindow()
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)
  }
}

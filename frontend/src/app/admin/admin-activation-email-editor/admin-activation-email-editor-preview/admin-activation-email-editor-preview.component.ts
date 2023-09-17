import {AfterViewInit, Component, computed, effect, ElementRef, signal, ViewChild} from '@angular/core';
import {WindowContent} from "../../../data/window-content";
import {W2kWindowFrameComponent} from "../../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {AdminActivationEmailPreviewProps} from "./admin-activation-email-preview-props";

@Component({
  selector: 'app-admin-activation-email-editor-preview',
  templateUrl: './admin-activation-email-editor-preview.component.html',
  styleUrls: ['./admin-activation-email-editor-preview.component.css']
})
export class AdminActivationEmailEditorPreviewComponent extends WindowContent<AdminActivationEmailPreviewProps, W2kWindowFrameComponent> implements AfterViewInit {

  @ViewChild('display')
  display!: ElementRef<HTMLDivElement>

  _content: string = ''
  set content(s: string) {
    this._content = s
  }

  override setParams(params: AdminActivationEmailPreviewProps) {
    this.content = params.content
    setTimeout(() => this.updatePreview())
  }

  public updatePreview() {
    if (this.display == null) return
    this.display.nativeElement.innerHTML = this._content
  }

  ngAfterViewInit(): void {
    this.windowFrame.onClose = () => this.closeWindow()
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)
    setTimeout(() => {
      this.setIcon("/assets/activation-email-s.png")
      this.setTitle("Activation email preview")
    })
  }
}

import {AfterViewInit, Component, inject, signal} from '@angular/core';
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {WindowService} from "../../service/window.service";
import {
  AdminActivationEmailEditorPreviewComponent
} from "./admin-activation-email-editor-preview/admin-activation-email-editor-preview.component";
import {
  AdminActivationEmailPreviewProps
} from "./admin-activation-email-editor-preview/admin-activation-email-preview-props";
import {ActivationService} from "../../service/activation.service";
import {PopupService} from "../../service/popup.service";

@Component({
  selector: 'app-admin-activation-email-editor',
  templateUrl: './admin-activation-email-editor.component.html',
  styleUrls: ['./admin-activation-email-editor.component.css']
})
export class AdminActivationEmailEditorComponent extends WindowContent<null, W2kWindowFrameComponent> implements AfterViewInit {

  private windowService = inject(WindowService)
  private activationService = inject(ActivationService)
  private popupService = inject(PopupService)

  private _content: string = ''

  set content(v: string) {
    this._content = v
  }

  get content(): string {
    return this._content;
  }

  ngAfterViewInit(): void {
    this.windowFrame.onClose = () => this.closeWindow()
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)
    setTimeout(() => {
      this.setIcon("/assets/activation-email-s.png")
      this.setTitle("Change activation email")
    })

    this.activationService.getActivationEmailTemplate().subscribe({
      next: value => {
        if (value != null)
          this.content = value.content
      }
    })
  }

  updateActivationEmail() {
    this.activationService.updateActivationEmailTemplate(this.content).subscribe({
      next: _ => {
        this.popupService.info(
          "Activation email changed",
          "The activation email has been updated."
        )
        this.closeWindow()
      }
    })
  }


  showPreview() {
    this.windowService.openApplication(
      AdminActivationEmailEditorPreviewComponent,
      new AdminActivationEmailPreviewProps(this.content),
      W2kWindowFrameComponent
    )
  }
}

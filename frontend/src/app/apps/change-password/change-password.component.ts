import {AfterViewInit, Component, inject} from '@angular/core';
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {AccountService} from "../../service/account.service";
import {PopupService} from "../../service/popup.service";

@Component({
    selector: 'app-change-password',
    templateUrl: './change-password.component.html',
    styleUrls: ['./change-password.component.css'],
    standalone: false
})
export class ChangePasswordComponent extends WindowContent<null, W2kWindowFrameComponent> implements AfterViewInit {
  private accountService = inject(AccountService)
  private popupService = inject(PopupService)

  public form = new FormGroup({
    oldPassword: new FormControl<string>('', Validators.required),
    newPassword: new FormControl<string>('', Validators.required),
  })

  ngAfterViewInit(): void {
    this.windowFrame.onClose = () => this.closeWindow()
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)

    setTimeout(() => {
      this.setTitle("Change password")
      this.setIcon("/assets/change-password-s.png")
    })
  }

  updatePassword() {
    let form = this.form.getRawValue()
    this.accountService.changePassword(form.oldPassword as string, form.newPassword as string).subscribe({
      next: _ => {
        this.popupService.info(
          "Password changed",
          "The password has been changed successfully."
          )
        this.closeWindow()
      },
      error: _ => {
        this.popupService.error(
          "Password change failed",
          "Password has not been changed, have you typed your old password correctly?"
        )
      }
    })
  }

}

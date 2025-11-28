import {AfterViewInit, Component, inject} from '@angular/core';
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {PopupService} from "../../service/popup.service";
import {RegistrationService} from "../../service/registration.service";
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";

@Component({
    selector: 'app-registration-form',
    templateUrl: './registration-form.component.html',
    styleUrls: ['./registration-form.component.css'],
    standalone: false
})
export class RegistrationFormComponent extends WindowContent<null, W2kWindowFrameComponent> implements AfterViewInit {

  public form = new FormGroup({
    username: new FormControl<string>('', Validators.required),
    email: new FormControl<string>('', Validators.required),
    password: new FormControl<string>('', Validators.required),
    confirmPassword: new FormControl<string>('', Validators.required),
  })

  private notificationService = inject(PopupService)
  private registrationService = inject(RegistrationService)

  public register() {
    let form = this.form.getRawValue()
    this.registrationService.register(
      form.username as string,
      form.email as string,
      form.password as string
    ).subscribe(
        (_) => {
          this.notificationService.info(
            "Account created",
            "The verification email has been send to your E-Mail address. You'll need to activate your account before you can start using the system."
          )
          this.closeWindow()
        },
        (e) => {
          let status = e.status

          switch (status) {
            case 400 : {
              this.notificationService.error("Bad request", "Either the specified email address is in use, or the data is invalid.")
            } break;
            case 504 : {
              this.notificationService.error("No connection", "The server does not respond. Try again later.")
            } break;
          }
        }
      )
  }

  ngAfterViewInit(): void {
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)
    this.windowFrame.onClose = () => this.closeWindow()
    setTimeout(() => {
      this.setTitle("Create a new account")
      this.wm
        .getDisplaySize()
        .ifPresent(([x,y]) => this.wm.setPosition(this.id, x/2, y/2, true))
    })
  }
}

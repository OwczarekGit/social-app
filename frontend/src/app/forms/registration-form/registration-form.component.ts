import {AfterViewInit, Component, ComponentRef, inject, ViewChild} from '@angular/core';
import {FormBuilder, FormControl, FormGroup, Validators} from "@angular/forms";
import {WindowComponent} from "../../ui-elements/window/window.component";
import {NotificationService} from "../../service/notification.service";
import {RegistrationService} from "../../service/registration.service";

@Component({
  selector: 'app-registration-form',
  templateUrl: './registration-form.component.html',
  styleUrls: ['./registration-form.component.css']
})
export class RegistrationFormComponent {
  public form = new FormGroup({
    email: new FormControl<string>('', Validators.required),
    password: new FormControl<string>('', Validators.required),
    confirmPassword: new FormControl<string>('', Validators.required),
  })

  private notificationService = inject(NotificationService)
  private registrationService = inject(RegistrationService)

  @ViewChild(WindowComponent)
  window!: WindowComponent

  public register() {
    let form = this.form.getRawValue()
    this.registrationService.register(form.email as string, form.password as string)
      .subscribe(
        (r) => {
          this.notificationService.info(
            "Account created",
            "The verification email has been send to your E-Mail address. You'll need to activate your account before you can start using the system."
          )
          this.closeWindow()
        },
        (e) => {
          let status = e.status

          switch (status) {
            case
            400 : {
              this.notificationService.error("Bad request", "Either the specified email address is in use, or the data is invalid.")
            } break;
            case 404 : {
              this.notificationService.error("No connection", "The server does not respond. Try again later.")
            } break;
          }
        }
      )
  }

  public closeWindow() {
    this.window.closeWindow()
  }
}

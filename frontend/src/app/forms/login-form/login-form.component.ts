import {AfterViewInit, Component, inject, ViewChild} from '@angular/core';
import {WindowComponent} from "../../ui-elements/window/window.component";
import {WindowService} from "../../service/window.service";
import {RegistrationFormComponent} from "../registration-form/registration-form.component";
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {LoginService} from "../../service/login.service";
import {PopupService} from "../../service/popup.service";
import {Router} from "@angular/router";

@Component({
  selector: 'app-login-form',
  templateUrl: './login-form.component.html',
  styleUrls: ['./login-form.component.css']
})
export class LoginFormComponent implements AfterViewInit {

  public form = new FormGroup({
    email: new FormControl<string>('', Validators.required),
    password: new FormControl<string>('', Validators.required),
  })

  @ViewChild(WindowComponent)
  window!: WindowComponent

  private windowService = inject(WindowService)
  private loginService = inject(LoginService)
  private notificationService = inject(PopupService)
  private router = inject(Router)

  ngAfterViewInit(): void {
    let [x,y] = this.windowService.getSurfaceSize()
    this.window.setPosition(x/2-411/2,y/2-230/2)
  }

  register() {
    this.windowService.openApplication(RegistrationFormComponent)
  }

  login() {
    let form = this.form.getRawValue()

    this.loginService.login(form.email as string, form.password as string).subscribe(
      (r) => {
        this.router.navigate(['/desktop'])
        this.window.closeWindow()
      },
      (e) => {
        let status = e.status

        switch (status) {
          case 403: {
            this.notificationService.error("Wrong password", "User with this E-Mail address exists, but the password does not match.")
          } break;
          case 404: {
            this.notificationService.error("User not found", "User with specified E-Mail address not found.")
          } break;
        }
      }
    )

  }
}

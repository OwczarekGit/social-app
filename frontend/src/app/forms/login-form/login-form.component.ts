import {AfterViewInit, Component, inject, ViewChild} from '@angular/core';
import {RegistrationFormComponent} from "../registration-form/registration-form.component";
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {LoginService} from "../../service/login.service";
import {PopupService} from "../../service/popup.service";
import {Router} from "@angular/router";
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {WindowService} from "../../service/window.service";
import {SoundService} from "../../service/sound.service";

@Component({
    selector: 'app-login-form',
    templateUrl: './login-form.component.html',
    styleUrls: ['./login-form.component.css'],
    standalone: false
})
export class LoginFormComponent extends WindowContent<null, W2kWindowFrameComponent> implements AfterViewInit {

  public form = new FormGroup({
    email: new FormControl<string>('', Validators.required),
    password: new FormControl<string>('', Validators.required),
  })

  private windowService = inject(WindowService)
  private soundService = inject(SoundService)
  private loginService = inject(LoginService)
  private notificationService = inject(PopupService)
  private router = inject(Router)

  ngAfterViewInit(): void {
    this.windowFrame.onClose = () => this.closeWindow()
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)

    setTimeout(() => {
      this.windowFrame.close = false
      this.setIcon("")
      this.setTitle("Log on to service")
      this.wm
        .getDisplaySize()
        .ifPresent(([x,y]) => this.wm.setPosition(this.id, x/2, y/2, true))
    })
  }

  register() {
    this.windowService.openApplication(RegistrationFormComponent, null, W2kWindowFrameComponent)
  }

  login() {
    let form = this.form.getRawValue()

    this.loginService.login(form.email as string, form.password as string).subscribe(
      (r) => {
        this.router.navigate(['/desktop'])
        this.soundService.login()
        this.closeWindow()
      },
      (e) => {
        let status = e.status

        switch (status) {
          case 400: {
            this.notificationService.error("Wrong password", "Invalid E-Mail address or password.")
          } break;
          case 504: {
            this.notificationService.error("Service unavailable", "The service does not respond.")
          } break;
        }
      }
    )

  }
}

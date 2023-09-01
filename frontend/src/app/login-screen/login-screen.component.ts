import {AfterViewInit, Component, inject, ViewContainerRef} from '@angular/core';
import {WindowService} from "../service/window.service";
import {LoginFormComponent} from "../forms/login-form/login-form.component";
import {AuthService} from "../service/auth.service";
import {Router} from "@angular/router";
import {NewWindowService} from "../service/new-window.service";
import {W2kWindowFrameComponent} from "../ui-elements/w2k-window-frame/w2k-window-frame.component";

@Component({
  selector: 'app-login-screen',
  templateUrl: './login-screen.component.html',
  styleUrls: ['./login-screen.component.css']
})
export class LoginScreenComponent implements AfterViewInit {

  public vcr = inject(ViewContainerRef)
  private windowService = inject(NewWindowService)
  private authService = inject(AuthService)
  private router = inject(Router)

  ngAfterViewInit(): void {
    this.windowService.setDisplay(this.vcr)
    setTimeout(() => {
      if (this.authService.isNotLoggedIn())
        this.windowService.openApplication(LoginFormComponent, null, W2kWindowFrameComponent)
      else
        this.router.navigate(['/desktop'])
    }, 0)
  }

}

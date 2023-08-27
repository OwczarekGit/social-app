import {AfterViewInit, Component, inject, ViewContainerRef} from '@angular/core';
import {WindowService} from "../service/window.service";
import {LoginFormComponent} from "../forms/login-form/login-form.component";
import {AuthService} from "../service/auth.service";
import {Router} from "@angular/router";

@Component({
  selector: 'app-login-screen',
  templateUrl: './login-screen.component.html',
  styleUrls: ['./login-screen.component.css']
})
export class LoginScreenComponent implements AfterViewInit {

  public vcr = inject(ViewContainerRef)
  private windowService = inject(WindowService)
  private authService = inject(AuthService)
  private router = inject(Router)

  ngAfterViewInit(): void {
    this.windowService.setSurface(this.vcr)
    setTimeout(() => {
      if (this.authService.isNotLoggedIn())
        this.windowService.openApplication(LoginFormComponent)
      else
        this.router.navigate(['/desktop'])
    }, 0)
  }

}

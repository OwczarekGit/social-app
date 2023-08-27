import {Component, inject} from '@angular/core';
import {LoginService} from "../../../../service/login.service";
import {ActivatedRoute, Router} from "@angular/router";

@Component({
  selector: 'app-start-menu',
  templateUrl: './start-menu.component.html',
  styleUrls: ['./start-menu.component.css'],
})
export class StartMenuComponent {
  public loginService = inject(LoginService)
  public router = inject(Router)


  logout() {
    this.loginService.logout().subscribe({complete: () => this.router.navigate(['/'])})

  }

}

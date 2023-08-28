import {Component, inject} from '@angular/core';
import {LoginService} from "../../../../service/login.service";
import {ActivatedRoute, Router} from "@angular/router";
import {WindowService} from "../../../../service/window.service";
import {PostWriterComponent} from "../../../../apps/post-writer/post-writer.component";

@Component({
  selector: 'app-start-menu',
  templateUrl: './start-menu.component.html',
  styleUrls: ['./start-menu.component.css'],
})
export class StartMenuComponent {
  public loginService = inject(LoginService)
  public router = inject(Router)
  public windowService = inject(WindowService)


  logout() {
    this.loginService.logout().subscribe({complete: () => this.router.navigate(['/'])})

  }

  openCreatePost() {
    this.windowService.openApplication(PostWriterComponent)
  }
}

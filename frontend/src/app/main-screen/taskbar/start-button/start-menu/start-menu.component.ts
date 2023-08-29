import {Component, inject} from '@angular/core';
import {LoginService} from "../../../../service/login.service";
import {ActivatedRoute, Router} from "@angular/router";
import {WindowService} from "../../../../service/window.service";
import {PostWriterComponent} from "../../../../apps/post-writer/post-writer.component";
import {FriendManagerComponent} from "../../../../apps/friend-manager/friend-manager.component";
import {PeopleSearcherComponent} from "../../../../apps/people-searcher/people-searcher.component";
import {NotificationCenterComponent} from "../../../../apps/notification-center/notification-center.component";

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

  openFriendManager() {
    this.windowService.openApplication(FriendManagerComponent)
  }

  openChangeUsername() {
    this.windowService.openApplication(NotificationCenterComponent)
  }

  openSearchFriends() {
    this.windowService.openApplication(PeopleSearcherComponent)
  }
}

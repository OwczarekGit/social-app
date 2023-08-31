import {Component, inject} from '@angular/core';
import {LoginService} from "../../../../service/login.service";
import {Router} from "@angular/router";
import {WindowService} from "../../../../service/window.service";
import {PostWriterComponent} from "../../../../apps/post-writer/post-writer.component";
import {FriendManagerComponent} from "../../../../apps/friend-manager/friend-manager.component";
import {PeopleSearcherComponent} from "../../../../apps/people-searcher/people-searcher.component";
import {ChangeUsernameComponent} from "../../../../apps/change-username/change-username.component";
import {ShareImageComponent} from "../../../../apps/share-image/share-image.component";
import {AuthService} from "../../../../service/auth.service";
import {AdminTagEditorComponent} from "../../../../admin/admin-tag-editor/admin-tag-editor.component";
import {NewWindowService} from "../../../../service/new-window.service";
import {W2kWindowFrameComponent} from "../../../../ui-elements/w2k-window-frame/w2k-window-frame.component";

@Component({
  selector: 'app-start-menu',
  templateUrl: './start-menu.component.html',
  styleUrls: ['./start-menu.component.css'],
})
export class StartMenuComponent {
  public loginService = inject(LoginService)
  public router = inject(Router)
  public windowService = inject(WindowService)
  public newWindowService = inject(NewWindowService)
  public authService = inject(AuthService)


  logout() {
    this.loginService.logout().subscribe({complete: () => this.router.navigate(['/'])})

  }

  openCreatePost() {
    this.newWindowService.openApplication(PostWriterComponent, null, W2kWindowFrameComponent)
  }

  openFriendManager() {
    this.windowService.openApplication(FriendManagerComponent)
  }

  openChangeUsername() {
    this.newWindowService.openApplication(ChangeUsernameComponent, null, W2kWindowFrameComponent)
  }

  openSearchFriends() {
    this.windowService.openApplication(PeopleSearcherComponent)
  }

  openShareImage() {
    this.windowService.openApplication(ShareImageComponent)
  }

  // ADMIN APPS
  openAdminManageTags() {
    this.windowService.openApplication(AdminTagEditorComponent)
  }
}

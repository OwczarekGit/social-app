import {Component, inject} from '@angular/core';
import {LoginService} from "../../../../service/login.service";
import {Router} from "@angular/router";
import {PostWriterComponent} from "../../../../apps/post-writer/post-writer.component";
import {FriendManagerComponent} from "../../../../apps/friend-manager/friend-manager.component";
import {PeopleSearcherComponent} from "../../../../apps/people-searcher/people-searcher.component";
import {ChangeUsernameComponent} from "../../../../apps/change-username/change-username.component";
import {ShareImageComponent} from "../../../../apps/share-image/share-image.component";
import {AuthService} from "../../../../service/auth.service";
import {AdminTagEditorComponent} from "../../../../admin/admin-tag-editor/admin-tag-editor.component";
import {WindowService} from "../../../../service/window.service";
import {W2kWindowFrameComponent} from "../../../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {MessengerComponent} from "../../../../apps/messenger/messenger.component";
import {WallpaperPickerComponent} from "../../../../apps/wallpaper-picker/wallpaper-picker.component";
import {NotificationService} from "../../../../service/notification.service";
import {
  AdminActivationEmailEditorComponent
} from "../../../../admin/admin-activation-email-editor/admin-activation-email-editor.component";
import {ChangePasswordComponent} from "../../../../apps/change-password/change-password.component";
import {AdminManageDomainsComponent} from "../../../../admin/admin-manage-domains/admin-manage-domains.component";

@Component({
  selector: 'app-start-menu',
  templateUrl: './start-menu.component.html',
  styleUrls: ['./start-menu.component.css'],
})
export class StartMenuComponent {
  public loginService = inject(LoginService)
  public router = inject(Router)
  public windowService = inject(WindowService)
  public authService = inject(AuthService)
  public notificationService = inject(NotificationService)


  logout() {
    this.loginService.logout().subscribe({complete: () => {
        this.windowService.openedApplications.forEach((_, i) => this.windowService.closeApplication(i))
        this.notificationService.unsubscribe()
        this.router.navigate(['/'])
      }})
  }

  openCreatePost() {
    this.windowService.openApplication(PostWriterComponent, null, W2kWindowFrameComponent)
  }

  openFriendManager() {
    this.windowService.openApplication(FriendManagerComponent, null, W2kWindowFrameComponent)
  }

  openChangeUsername() {
    this.windowService.openApplication(ChangeUsernameComponent, null, W2kWindowFrameComponent)
  }

  openChangePassword() {
    this.windowService.openApplication(ChangePasswordComponent, null, W2kWindowFrameComponent)
  }

  openSearchFriends() {
    this.windowService.openApplication(PeopleSearcherComponent, null, W2kWindowFrameComponent)
  }

  openShareImage() {
    this.windowService.openApplication(ShareImageComponent, null, W2kWindowFrameComponent)
  }

  openMessenger() {
    this.windowService.openApplication(MessengerComponent, null, W2kWindowFrameComponent)
  }

  openWallpaperSettings() {
    this.windowService.openApplication(WallpaperPickerComponent, null, W2kWindowFrameComponent)
  }

  // ADMIN APPS
  openAdminManageTags() {
    this.windowService.openApplication(AdminTagEditorComponent, null, W2kWindowFrameComponent)
  }

  openChangeActivationEmail() {
    this.windowService.openApplication(AdminActivationEmailEditorComponent, null, W2kWindowFrameComponent)
  }

  openManageDomains() {
    this.windowService.openApplication(AdminManageDomainsComponent, null, W2kWindowFrameComponent)
  }
}

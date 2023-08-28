import {Component, inject, ViewChild} from '@angular/core';
import {WindowComponent} from "../../ui-elements/window/window.component";
import {WindowService} from "../../service/window.service";

@Component({
  selector: 'app-friend-manager',
  templateUrl: './friend-manager.component.html',
  styleUrls: ['./friend-manager.component.css']
})
export class FriendManagerComponent {
  protected readonly Array = Array;
  public friendsTabOpened: boolean = true
  protected windowService = inject(WindowService)

  @ViewChild(WindowComponent)
  window!: WindowComponent


  openFriendsTab() {
    this.friendsTabOpened = true
  }

  openRequestTab() {
    this.friendsTabOpened = false
  }

  public close() {
    this.window.closeWindow()
  }
}

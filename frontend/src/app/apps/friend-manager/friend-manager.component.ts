import {Component, inject, ViewChild} from '@angular/core';
import {WindowComponent} from "../../ui-elements/window/window.component";
import {WindowService} from "../../service/window.service";
import {FriendRequest} from "../../data/friend-request";
import {ListDisplay} from "../../data/list-display";
import {FriendService} from "../../service/friend.service";

@Component({
  selector: 'app-friend-manager',
  templateUrl: './friend-manager.component.html',
  styleUrls: ['./friend-manager.component.css']
})
export class FriendManagerComponent {
  protected readonly Array = Array;
  public friendsTabOpened: boolean = true
  protected windowService = inject(WindowService)
  private friendService = inject(FriendService)

  @ViewChild(WindowComponent)
  window!: WindowComponent

  selectedFriendRequest: FriendRequest | null = null

  friendRequests: FriendRequest[] = []

  openFriendsTab() {
    this.selectedFriendRequest = null
    this.friendsTabOpened = true

  }

  openRequestTab() {
    this.selectedFriendRequest = null
    this.friendsTabOpened = false

    this.friendService.getPendingFriendRequests().subscribe({
      next: value => {
        this.friendRequests = value.map(v => new FriendRequest(v.user_id, v.username))
      }
    })
  }

  public close() {
    this.window.closeWindow()
  }

  setSelectedFriendRequest(e: ListDisplay) {
    this.selectedFriendRequest = e as FriendRequest
  }

  acceptFriendRequest() {
    if (this.selectedFriendRequest != null)
      this.friendService.acceptFriendRequest(this.selectedFriendRequest.user_id).subscribe({complete: () => {
        this.openRequestTab()
      }})
  }

  denyFriendRequest() {
    if (this.selectedFriendRequest != null)
      this.friendService.denyFriendRequest(this.selectedFriendRequest.user_id).subscribe({
        complete: () => this.openRequestTab()
      })
  }

  removeFriend() {
    let i = this.friendRequests.findIndex(u => u.user_id == this.selectedFriendRequest?.user_id)
    this.friendRequests.splice(i,1)
    this.friendRequests = [...this.friendRequests]
  }
}

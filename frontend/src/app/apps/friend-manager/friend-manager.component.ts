import {AfterViewInit, Component, inject} from '@angular/core';
import {FriendRequest} from "../../data/friend-request";
import {ListDisplay} from "../../data/list-display";
import {FriendService} from "../../service/friend.service";
import {Profile} from "../../data/profile";
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";

@Component({
  selector: 'app-friend-manager',
  templateUrl: './friend-manager.component.html',
  styleUrls: ['./friend-manager.component.css']
})
export class FriendManagerComponent extends WindowContent<null, W2kWindowFrameComponent> implements AfterViewInit {

  protected readonly Array = Array;
  public friendsTabOpened: boolean = true
  private friendService = inject(FriendService)

  selectedFriendRequest: FriendRequest | null = null
  selectedFriendProfile: Profile | null = null

  friendRequests: FriendRequest[] = []
  friendProfiles: Profile[] = []

  constructor() {
    super()
    this.openFriendsTab()
  }

  openFriendsTab() {
    this.selectedFriendProfile = null
    this.selectedFriendRequest = null
    this.friendsTabOpened = true

    this.friendService.getFriendList().subscribe({
      next: value => {
        this.friendProfiles = value.map(p => new Profile(p.id, p.username))
      }
    })
  }

  openRequestTab() {
    this.selectedFriendProfile = null
    this.selectedFriendRequest = null
    this.friendsTabOpened = false

    this.friendService.getPendingFriendRequests().subscribe({
      next: value => {
        this.friendRequests = value.map(v => new FriendRequest(v.user_id, v.username))
      }
    })
  }

  public close() {
    this.closeWindow()
  }

  setSelectedFriendRequest(e: ListDisplay) {
    this.selectedFriendRequest = e as FriendRequest
  }

  setSelectedFriendProfile(e: ListDisplay) {
    this.selectedFriendProfile = e as Profile
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
    let i = this.friendProfiles.findIndex(u => u.id == this.selectedFriendProfile?.id)
    this.friendProfiles.splice(i,1)
    this.friendProfiles = [...this.friendProfiles]
  }

  ngAfterViewInit(): void {
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)
    this.windowFrame.onClose = () => this.closeWindow()

    setTimeout(() => {
      this.setIcon("/assets/user-icon-s.png")
      this.setTitle("Manage friends")
    })
  }
}

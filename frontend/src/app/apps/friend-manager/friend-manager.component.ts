import {AfterViewInit, Component, inject} from '@angular/core';
import {FriendRequest} from "../../data/friend-request";
import {ListDisplay} from "../../data/list-display";
import {FriendService} from "../../service/friend.service";
import {Profile} from "../../data/profile";
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {PopupService} from "../../service/popup.service";
import {filter} from "rxjs";
import {DomainService} from "../../service/domain.service";

@Component({
  selector: 'app-friend-manager',
  templateUrl: './friend-manager.component.html',
  styleUrls: ['./friend-manager.component.css']
})
export class FriendManagerComponent extends WindowContent<null, W2kWindowFrameComponent> implements AfterViewInit {

  protected readonly Array = Array;
  public friendsTabOpened: boolean = true
  private friendService = inject(FriendService)
  private popupService = inject(PopupService)
  private domainService = inject(DomainService)

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
        this.friendProfiles = value.map(p => new Profile(p.user_id, p.username, p.picture_url, this.domainService.imageDomain))
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
    if (this.selectedFriendProfile == null) return
    console.log(this.selectedFriendProfile)
    this.friendService.removeFriend(this.selectedFriendProfile?.user_id).subscribe({
      next: value => {
        let i = this.friendProfiles.findIndex(u => u.user_id == this.selectedFriendProfile?.user_id)
        this.friendProfiles.splice(i,1)
        this.friendProfiles = [...this.friendProfiles]
        this.popupService.info("Friend removed ", "You are no longer friends.")
      },
      error: _ => {
        this.popupService.info("Error removing friend", "There was an error while removing friend.")
      }
    })
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

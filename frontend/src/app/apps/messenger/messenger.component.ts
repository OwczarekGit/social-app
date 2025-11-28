import {AfterViewInit, Component, inject, signal} from '@angular/core';
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {WindowContent} from "../../data/window-content";
import {FriendService} from "../../service/friend.service";
import {Profile} from "../../data/profile";
import {ListDisplay} from "../../data/list-display";
import {ProfileService} from "../../service/profile.service";

@Component({
    selector: 'app-messenger',
    templateUrl: './messenger.component.html',
    styleUrls: ['./messenger.component.css'],
    standalone: false
})
export class MessengerComponent extends WindowContent<null, W2kWindowFrameComponent> implements AfterViewInit {
  public friendService = inject(FriendService)

  public friends = signal<Profile[]>([])
  public profileService = inject(ProfileService)

  public selectedProfile!: Profile

  public myProfile!: Profile

  constructor() {
    super();
    this.friendService.getFriendList().subscribe({
      next: value => {
        this.friends.set(value.map(v => new Profile(v.user_id, v.username, v.picture_url)))
        let p = this.friends()[0]
        if (p != null)
          this.selectedProfile = p
      }
    })

    this.profileService.getMyProfile().subscribe({
      next: v => {
        this.myProfile = new Profile(v.user_id, v.username, v.picture_url)
      }
    })
  }

  ngAfterViewInit(): void {
    this.windowFrame.onClose = () => this.closeWindow()
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)

    setTimeout(() => {
      this.setIcon("/assets/messenger-s.png")
      this.setTitle("Messenger")
    })
  }


  switchChat(profile: ListDisplay) {
    this.selectedProfile = profile as Profile
  }
}

import {AfterViewInit, Component, inject, ViewChild} from '@angular/core';
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {FriendService} from "../../service/friend.service";
import {SearchNonFriendResult} from "../../data/search-non-friend-result";
import {ListDisplay} from "../../data/list-display";
import {WindowComponent} from "../../ui-elements/window/window.component";
import {PopupService} from "../../service/popup.service";
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";

@Component({
  selector: 'app-people-searcher',
  templateUrl: './people-searcher.component.html',
  styleUrls: ['./people-searcher.component.css']
})
export class PeopleSearcherComponent extends WindowContent<null, W2kWindowFrameComponent> implements AfterViewInit {

  friendService = inject(FriendService)
  popupService = inject(PopupService)

  public form = new FormGroup({
    phrase: new FormControl<string>('', Validators.required)
  })

  public searchResults: SearchNonFriendResult[] = []
  public selected: SearchNonFriendResult | null = null

  public performSearch() {
    let form = this.form.getRawValue()
    this.friendService.searchNonFriends(form.phrase as string).subscribe({
      next: value => {
        this.searchResults = value.map(f => new SearchNonFriendResult(f.user_id, f.username))
      }
    })
  }

  changeSelected($event: ListDisplay) {
    this.selected = $event as SearchNonFriendResult
  }

  close() {
    this.closeWindow()
  }

  invite() {
    if (this.selected == null) return

    this.friendService.sendFriendRequest(this.selected.user_id).subscribe(
      (next) => {
        this.popupService.info(
          "Invitation sent",
          `The friend request has been sent to ${this.selected?.username}. ` +
          "You will become friends when the request is accepted."
        )

        this.form.reset()
        this.searchResults.splice(this.searchResults.findIndex(f => f.user_id == this.selected?.user_id),1)
        this.selected = null

      })
  }

  ngAfterViewInit(): void {
    this.windowFrame.onClose = () => this.closeWindow()
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)
    setTimeout(() => {
      this.setIcon("/assets/search-friends-s.png")
      this.setTitle("Search for friends")
    })
  }
}

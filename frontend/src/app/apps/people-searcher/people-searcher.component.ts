import { AfterViewInit, Component, DestroyRef, inject } from "@angular/core";
import { FormControl, FormGroup, Validators } from "@angular/forms";
import { FriendService } from "../../service/friend.service";
import { SearchNonFriendResult } from "../../data/search-non-friend-result";
import { ListDisplay } from "../../data/list-display";
import { PopupService } from "../../service/popup.service";
import { WindowContent } from "../../data/window-content";
import { W2kWindowFrameComponent } from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import { WindowService } from "../../service/window.service";
import { UserProfileComponent } from "../user-profile/user-profile.component";
import { debounceTime, filter, of, switchMap, withLatestFrom } from "rxjs";
import { takeUntilDestroyed } from "@angular/core/rxjs-interop";

@Component({
  selector: "app-people-searcher",
  templateUrl: "./people-searcher.component.html",
  styleUrls: ["./people-searcher.component.css"],
  standalone: false,
})
export class PeopleSearcherComponent
  extends WindowContent<null, W2kWindowFrameComponent>
  implements AfterViewInit
{
  private readonly friendService = inject(FriendService);
  private readonly popupService = inject(PopupService);
  private readonly windowService = inject(WindowService);
  private readonly destroyRef = inject(DestroyRef);

  protected readonly phrase: FormControl<string> = new FormControl("", {
    nonNullable: true,
    validators: [Validators.required],
  });

  public searchResults: SearchNonFriendResult[] = [];
  public selected: SearchNonFriendResult | null = null;

  changeSelected($event: ListDisplay) {
    this.selected = $event as SearchNonFriendResult;
  }

  close() {
    this.closeWindow();
  }

  invite() {
    if (this.selected == null) return;

    this.friendService
      .sendFriendRequest(this.selected.user_id)
      .subscribe((next) => {
        this.popupService.info(
          "Invitation sent",
          `The friend request has been sent to ${this.selected?.username}. ` +
            "You will become friends when the request is accepted.",
        );

        this.phrase.reset();
        this.searchResults.splice(
          this.searchResults.findIndex(
            (f) => f.user_id == this.selected?.user_id,
          ),
          1,
        );
        this.selected = null;
      });
  }

  ngAfterViewInit(): void {
    this.windowFrame.onClose = () => this.closeWindow();
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id);
    setTimeout(() => {
      this.setIcon("/assets/search-friends-s.png");
      this.setTitle("Search for friends");
    });

    this.phrase.valueChanges
      .pipe(
        debounceTime(300),
        switchMap((phrase) => this.friendService.searchNonFriends(phrase)),
        takeUntilDestroyed(this.destroyRef),
      )
      .subscribe({
        next: (response) =>
          (this.searchResults = response.map(
            (result) =>
              new SearchNonFriendResult(result.user_id, result.username),
          )),
      });
  }

  showProfile() {
    this.windowService.openApplication(
      UserProfileComponent,
      this.selected?.user_id,
      W2kWindowFrameComponent,
    );
  }
}

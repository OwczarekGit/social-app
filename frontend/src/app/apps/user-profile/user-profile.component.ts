import {AfterViewInit, Component, inject, OnDestroy, signal} from '@angular/core';
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {ProfileService} from "../../service/profile.service";
import {Profile} from "../../data/profile";
import {Post} from "../../data/post";
import {PostService} from "../../service/post.service";
import {EventService} from "../../service/event.service";
import {Subscription} from "rxjs";

@Component({
    selector: 'app-user-profile',
    templateUrl: './user-profile.component.html',
    styleUrls: ['./user-profile.component.css'],
    standalone: false
})
export class UserProfileComponent extends WindowContent<number, W2kWindowFrameComponent> implements AfterViewInit, OnDestroy {
  private profileService = inject(ProfileService)
  private postService = inject(PostService)
  private eventService = inject(EventService)

  private postEditSub!: Subscription
  private postDeleteSub!: Subscription
  private postCreateSub!: Subscription

  public currentUserProfile = signal<Profile | null>(null)
  public profile = signal<Profile | null>(null)
  public posts = signal<Post[]>([])

  ngAfterViewInit(): void {
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)
    this.windowFrame.onClose = () => this.closeWindow()
    setTimeout(() => {
      this.setIcon("/assets/user-icon-s.png")
    })

    this.postEditSub = this.eventService.postEditedSub.subscribe({
      next: p => {
        this.posts.update(old => {
          let i = old.findIndex(o => o.id == p.id)
          if (i != -1) {
            old[i] = p
          }
          return [...old]
        })
      }
    })

    this.postDeleteSub = this.eventService.postDeletedSub.subscribe({
      next: p => {
        this.posts.update(old => {
          let i = old.findIndex(o => o.id == p.id)
          if (i != -1) {
            old.splice(i, 1)
          }
          return [...old]
        })
      }
    })

    this.postCreateSub = this.eventService.postCreatedSub.subscribe({
      next: p => {
        this.posts.update(old => {
          old.unshift(p)
          return [...old]
        })
      }
    })

  }

  override setParams(params: number) {
    this.profileService.getProfileForUserId(params).subscribe({
      next: value => {
        this.profile.set(new Profile(value.user_id, value.username, value.picture_url))
        this.setTitle(`${value.username}'s profile`)

        this.postService.getPostsForUser(value.user_id).subscribe({
          next: posts => this.posts.set(posts.map(p =>
            new Post(p.id, p.author_id, p.author_username, p.author_picture_url, p.content, p.date)))
        })}
    })

    this.profileService.getMyProfile().subscribe({
      next: value => this.currentUserProfile.set(new Profile(value.user_id, value.username, value.picture_url))
    })
  }

  ngOnDestroy(): void {
    this.postEditSub.unsubscribe()
    this.postDeleteSub.unsubscribe()
    this.postCreateSub.unsubscribe()
  }
}

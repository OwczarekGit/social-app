import {AfterViewInit, Component, inject, signal} from '@angular/core';
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {ProfileService} from "../../service/profile.service";
import {Profile} from "../../data/profile";
import {Post} from "../../data/post";
import {PostService} from "../../service/post.service";

@Component({
  selector: 'app-user-profile',
  templateUrl: './user-profile.component.html',
  styleUrls: ['./user-profile.component.css']
})
export class UserProfileComponent extends WindowContent<number, W2kWindowFrameComponent> implements AfterViewInit {
  private profileService = inject(ProfileService)
  private postService = inject(PostService)

  public profile = signal<Profile | null>(null)
  public posts = signal<Post[]>([])

  ngAfterViewInit(): void {
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)
    this.windowFrame.onClose = () => this.closeWindow()
    setTimeout(() => {
      this.setIcon("/assets/user-icon-s.png")
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
        })

      }
    })
  }
}

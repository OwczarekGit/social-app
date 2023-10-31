import {Component, inject, Input} from '@angular/core';
import {Post} from "../../../data/post";
import {WindowService} from "../../../service/window.service";
import {UserProfileComponent} from "../user-profile.component";
import {W2kWindowFrameComponent} from "../../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {PostWriterComponent} from "../../post-writer/post-writer.component";
import {Some} from "option-value";
import {Profile} from "../../../data/profile";
import {PostService} from "../../../service/post.service";
import {EventService} from "../../../service/event.service";
import {PopupService} from "../../../service/popup.service";

@Component({
  selector: 'app-user-profile-post',
  templateUrl: './user-profile-post.component.html',
  styleUrls: ['./user-profile-post.component.css']
})
export class UserProfilePostComponent {
  private windowService = inject(WindowService)
  private postService = inject(PostService)
  private eventService = inject(EventService)
  private popupService = inject(PopupService)

  @Input('post')
  post!: Post | null

  @Input('profile')
  profile!: Profile | null

  openProfile() {
    this.windowService.openApplication(UserProfileComponent, this.post?.author_id, W2kWindowFrameComponent)
  }

  editPost() {
    this.windowService.openApplication(PostWriterComponent, Some(this.post), W2kWindowFrameComponent)
  }

  deletePost() {
    this.postService.deletePost(this.post?.id as number).subscribe({
      next: _ => {
        this.popupService.info("Post deleted", "The post has been deleted.")
        this.eventService.emitPostDeleted(this.post as Post)
      }
    })
  }
}

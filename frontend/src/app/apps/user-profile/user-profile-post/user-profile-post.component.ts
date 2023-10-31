import {Component, inject, Input} from '@angular/core';
import {Post} from "../../../data/post";
import {WindowService} from "../../../service/window.service";
import {UserProfileComponent} from "../user-profile.component";
import {W2kWindowFrameComponent} from "../../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {PostWriterComponent} from "../../post-writer/post-writer.component";
import {Some} from "option-value";

@Component({
  selector: 'app-user-profile-post',
  templateUrl: './user-profile-post.component.html',
  styleUrls: ['./user-profile-post.component.css']
})
export class UserProfilePostComponent {
  private windowService = inject(WindowService)

  @Input('post')
  post!: Post | null

  openProfile() {
    this.windowService.openApplication(UserProfileComponent, this.post?.author_id, W2kWindowFrameComponent)
  }

  // TODO: After the post is updated it does not refresh in the ui until reopened.
  editPost(postId?: number, content?: string) {
    this.windowService.openApplication(PostWriterComponent, Some([postId as number, content as string]), W2kWindowFrameComponent)
  }
}

import {AfterViewInit, Component, inject} from '@angular/core';
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {PostService} from "../../service/post.service";
import {PopupService} from "../../service/popup.service";
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {None, Option, Some} from "option-value";
import {EventService} from "../../service/event.service";
import {Post} from "../../data/post";

@Component({
  selector: 'app-post-writer',
  templateUrl: './post-writer.component.html',
  styleUrls: ['./post-writer.component.css']
})
export class PostWriterComponent extends WindowContent<Option<Post>, W2kWindowFrameComponent> implements AfterViewInit {

  public form = new FormGroup({
    content: new FormControl<string>('', Validators.required)
  })

  public postService = inject(PostService)
  public popupService = inject(PopupService)
  public eventService = inject(EventService)

  public editedPost: Option<Post> = None()

  public publishPost(){
    let form = this.form.getRawValue()
    this.postService.writePost(form.content as string).subscribe(value => {
        this.popupService.info("Published", "Your post has been published successfully.")
        this.eventService.emitPostCreated(value)
        this.closeWindow()
      },
      error => console.log(error)
    )
  }

  override setParams(params: Option<Post>) {
    params.ifPresent(p => {
      this.form.controls.content.setValue(p.content)
      this.editedPost = Some(p)
    })
  }

  ngAfterViewInit(): void {
    this.windowFrame.onFocus = () => {
      this.wm.focusApplication(this.id)
    }

    this.windowFrame.onClose = () => {
      this.closeWindow()
    }

    setTimeout(() => {
      this.setIcon("/assets/write-post-s.png")
      this.setTitle("Create post")
    })
  }

  updatePost() {
    let form = this.form.getRawValue().content as string
    this.editedPost
      .ifPresent(p =>
        this.postService.editPost(p.id, form)
          .subscribe({next: _ => {
              this.popupService.info("Post updated", "The post has been updated")
              this.eventService.emitPostEdited(new Post(p.id, p.author_id, p.author_username, p.author_picture_url, form, p.date))
              this.closeWindow()
          }})
      )
  }
}

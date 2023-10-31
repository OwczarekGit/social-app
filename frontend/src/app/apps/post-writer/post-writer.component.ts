import {AfterViewInit, Component, inject} from '@angular/core';
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {PostService} from "../../service/post.service";
import {PopupService} from "../../service/popup.service";
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {None, Option, Some} from "option-value";

@Component({
  selector: 'app-post-writer',
  templateUrl: './post-writer.component.html',
  styleUrls: ['./post-writer.component.css']
})
export class PostWriterComponent extends WindowContent<Option<[number, string]>, W2kWindowFrameComponent> implements AfterViewInit {

  public form = new FormGroup({
    content: new FormControl<string>('', Validators.required)
  })

  public postService = inject(PostService)
  public popupService = inject(PopupService)
  public editMode: boolean = false
  public editPostId: Option<number> = None()

  public publishPost(){
    let form = this.form.getRawValue()
    this.postService.writePost(form.content as string).subscribe(value => {
      this.popupService.info("Published", "Your post has been published successfully.")
      this.closeWindow()
    },
        error => console.log(error)
    )
  }

  override setParams(params: Option<[number, string]>) {
    params.ifPresent(([id, content]) => {
      this.editMode = true
      this.editPostId = Some(id)
      this.form.controls.content.setValue(content)
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
    this.editPostId
      .ifPresent(id =>
        this.postService.editPost(id, this.form.getRawValue().content as string)
          .subscribe({next: _ => {
              this.closeWindow()
              this.popupService.info("Post updated", "The post has been updated")
          }})
      )
  }
}

import {AfterViewInit, Component, inject} from '@angular/core';
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {PostService} from "../../service/post.service";
import {PopupService} from "../../service/popup.service";
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";

@Component({
  selector: 'app-post-writer',
  templateUrl: './post-writer.component.html',
  styleUrls: ['./post-writer.component.css']
})
export class PostWriterComponent extends WindowContent<null, W2kWindowFrameComponent> implements AfterViewInit {

  public form = new FormGroup({
    content: new FormControl<string>('', Validators.required)
  })

  public postService = inject(PostService)
  public popupService = inject(PopupService)

  public publishPost(){
    let form = this.form.getRawValue()
    this.postService.writePost(form.content as string).subscribe(value => {
      this.popupService.info("Published", "Your post has been published successfully.")
      this.closeWindow()
    },
        error => console.log(error)
    )
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
}

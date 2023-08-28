import {Component, inject, ViewChild} from '@angular/core';
import {WindowComponent} from "../../ui-elements/window/window.component";
import {WindowService} from "../../service/window.service";
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {PostService} from "../../service/post.service";
import {PopupService} from "../../service/popup.service";

@Component({
  selector: 'app-post-writer',
  templateUrl: './post-writer.component.html',
  styleUrls: ['./post-writer.component.css']
})
export class PostWriterComponent {

  public form = new FormGroup({
    content: new FormControl<string>('', Validators.required)
  })

  public windowService = inject(WindowService)
  public postService = inject(PostService)
  public popupService = inject(PopupService)

  @ViewChild(WindowComponent)
  window!: WindowComponent

  public close() {
    this.window.closeWindow()
  }

  public publishPost(){
    let form = this.form.getRawValue()
    this.postService.writePost(form.content as string).subscribe(value => {
      this.popupService.info("Published", "Your post has been published successfully.")
      this.close()
    },
        error => console.log(error)
    )
  }

}

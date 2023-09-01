import {AfterViewInit, Component, inject, signal, ViewChild} from '@angular/core';
import {Tag} from "../../data/tag";
import {ImageService} from "../../service/image.service";
import {PopupService} from "../../service/popup.service";
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {NewWindowService} from "../../service/new-window.service";
import {TagPickerComponent} from "../tag-picker/tag-picker.component";
import {TagPickerParams} from "../tag-picker/tag-picker-params";

@Component({
  selector: 'app-share-image',
  templateUrl: './share-image.component.html',
  styleUrls: ['./share-image.component.css']
})
export class ShareImageComponent extends WindowContent<null, W2kWindowFrameComponent> implements AfterViewInit{
  imageService = inject(ImageService)
  popupService = inject(PopupService)


  tags: Tag[] = []

  preview: string = ''

  file = signal<File | null>(null)

  public share() {
    this.imageService.uploadImage(this.title, this.tags.map(t => t.name), this.file() as File).subscribe({
      next: _ => {
        this.popupService.info("Image shared", "Your image has been shared.")
        this.close()
      },
      error: _ => {
        this.popupService.error(
          "Error sharing image",
          "There was an error sharing image, is the image valid and less than 5MB?")
      }
    })
  }

  public close() {
    this.closeWindow()
  }

  selectFile(ev: any) {
    this.file.set(ev.target.files[0])

    if (this.file() != null) {
      let self = this
      let fr = new FileReader()
      fr.onload = (e) => {
        self.preview = e.target?.result as string
      }

      fr.readAsDataURL(this.file() as File)
    }
  }

  setTags(tags: Tag[]) {
    this.tags = tags
  }

  openPicker() {
    this.wm.openApplication(
      TagPickerComponent,
      new TagPickerParams(this.tags, (newTags: Tag[]) => {
        this.tags = newTags
      }),
      W2kWindowFrameComponent
    )
  }

  ngAfterViewInit(): void {
    this.windowFrame.onClose = () => this.close()
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)

    setTimeout(() => {
      this.setIcon("/assets/share-image-s.png")
      this.setTitle("Share image")
    })
  }
}

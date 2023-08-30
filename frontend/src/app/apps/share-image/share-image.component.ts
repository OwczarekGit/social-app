import {Component, inject, signal, ViewChild} from '@angular/core';
import {WindowComponent} from "../../ui-elements/window/window.component";
import {Tag} from "../../data/tag";
import {ImageService} from "../../service/image.service";
import {PopupService} from "../../service/popup.service";

@Component({
  selector: 'app-share-image',
  templateUrl: './share-image.component.html',
  styleUrls: ['./share-image.component.css']
})
export class ShareImageComponent {
  @ViewChild(WindowComponent)
  window!: WindowComponent

  imageService = inject(ImageService)
  popupService = inject(PopupService)

  tagPickerOpened: boolean = false

  tags: Tag[] = []
  title: string = ""

  preview: string = ''

  file = signal<File | null>(null)

  public share() {
    this.imageService.uploadImage(this.title, this.tags.map(t => t.name), this.file() as File).subscribe({
      complete: () => {
        this.popupService.info("Image shared", "Your image has been shared.")
        this.close()
      }
    })
  }

  public close() {
    this.window.closeWindow()
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
    this.tagPickerOpened = false
  }

  openPicker() {
    this.tagPickerOpened = true
  }
}

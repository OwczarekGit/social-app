import {Component, inject, signal, ViewChild} from '@angular/core';
import {WindowComponent} from "../../ui-elements/window/window.component";
import {Tag} from "../../data/tag";
import {ImageService} from "../../service/image.service";

@Component({
  selector: 'app-share-image',
  templateUrl: './share-image.component.html',
  styleUrls: ['./share-image.component.css']
})
export class ShareImageComponent {
  @ViewChild(WindowComponent)
  window!: WindowComponent

  imageService = inject(ImageService)

  tagPickerOpened: boolean = false

  tags: Tag[] = [
    new Tag("Cute"),
    new Tag("Cat"),
  ]

  preview: string = ''

  file = signal<File | null>(null)

  public share() {
    this.imageService.uploadImage("My cute image", this.tags.map(t => t.name), this.file() as File).subscribe({
      complete: () => console.log("uploaded")
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

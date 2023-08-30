import {Component, EventEmitter, inject, Input, Output, ViewChild} from '@angular/core';
import {Tag} from "../../data/tag";
import {WindowComponent} from "../../ui-elements/window/window.component";
import {WindowService} from "../../service/window.service";
import {ImageService} from "../../service/image.service";
import {ListDisplay} from "../../data/list-display";

@Component({
  selector: 'app-tag-picker',
  templateUrl: './tag-picker.component.html',
  styleUrls: ['./tag-picker.component.css']
})
export class TagPickerComponent {

  windowService = inject(WindowService)
  imageService = inject(ImageService)

  @ViewChild(WindowComponent)
  window!: WindowComponent

  @Output()
  onConfirm: EventEmitter<Tag[]> = new EventEmitter<Tag[]>()

  newTagName: string = ''

  @Input()
  selectedTags: Tag[] = []

  allTags: Tag[] = []

  selectedAllTag: Tag | null = null
  selectedSelectedTag: Tag | null = null

  constructor() {
    this.imageService.getAllTags().subscribe({
      next: value => {
        this.allTags = value.map(v => new Tag(v.name))
      }
    })
  }

  public addTag() {
    if (this.newTagName.trim() == "") return
    this.selectedTags.push(new Tag(this.newTagName.trim()))
    this.newTagName = ''
  }

  public confirm() {
    this.onConfirm.emit(this.selectedTags)
  }

  changeAllTagsSelection($event: ListDisplay) {
    this.selectedAllTag = $event as Tag
  }

  changeSelectedTagsSelection($event: ListDisplay) {
    this.selectedSelectedTag = $event as Tag
  }

  moveSelectedAllTag() {
    if (this.selectedAllTag != null) {
      let selected = this.allTags.splice(this.allTags.findIndex(t => t.name == this.selectedAllTag?.name),1)[0]
      this.selectedTags.push(selected)
      this.selectedAllTag = null
    }
  }

  moveSelectedSelectedTag() {
    if (this.selectedSelectedTag != null) {
      let selected = this.selectedTags.splice(this.selectedTags.findIndex(t => t.name == this.selectedSelectedTag?.name),1)[0]
      this.allTags.push(selected)
      this.selectedSelectedTag = null
    }
  }

}

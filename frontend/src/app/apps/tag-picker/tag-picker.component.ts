import {AfterViewInit, Component, EventEmitter, inject, Output } from '@angular/core';
import {Tag} from "../../data/tag";
import {WindowService} from "../../service/window.service";
import {ImageService} from "../../service/image.service";
import {ListDisplay} from "../../data/list-display";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {WindowContent} from "../../data/window-content";
import {TagPickerParams} from "./tag-picker-params";

@Component({
  selector: 'app-tag-picker',
  templateUrl: './tag-picker.component.html',
  styleUrls: ['./tag-picker.component.css']
})
export class TagPickerComponent extends WindowContent<TagPickerParams, W2kWindowFrameComponent> implements AfterViewInit {

  windowService = inject(WindowService)
  imageService = inject(ImageService)

  newTagName: string = ''

  selectedTags: Tag[] = []

  allTags: Tag[] = []

  selectedAllTag: Tag | null = null
  selectedSelectedTag: Tag | null = null

  params!: TagPickerParams

  override setParams(params: TagPickerParams) {
    this.selectedTags = params.currentTags
    this.params = params
  }

  constructor() {
    super()
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
    this.params.resultTags(this.selectedTags)
    this.closeWindow()
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

  ngAfterViewInit(): void {
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)
    this.windowFrame.onClose = () => this.closeWindow()

    setTimeout(() => {
      this.windowFrame.close = false
      this.setTitle("Select tags")
      this.setIcon("/assets/tag-s.png")
    })
  }

}

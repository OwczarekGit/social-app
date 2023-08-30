import {Component, EventEmitter, inject, Input, Output, ViewChild} from '@angular/core';
import {Tag} from "../../data/tag";
import {WindowComponent} from "../../ui-elements/window/window.component";
import {WindowService} from "../../service/window.service";

@Component({
  selector: 'app-tag-picker',
  templateUrl: './tag-picker.component.html',
  styleUrls: ['./tag-picker.component.css']
})
export class TagPickerComponent {

  windowService = inject(WindowService)

  @ViewChild(WindowComponent)
  window!: WindowComponent

  @Output()
  onConfirm: EventEmitter<Tag[]> = new EventEmitter<Tag[]>()

  newTagName: string = ''

  @Input()
  selectedTags: Tag[] = []

  allTags: Tag[] = []

  public addTag() {
    this.selectedTags.push(new Tag(this.newTagName))
    this.newTagName = ''
  }

  public confirm() {
    this.onConfirm.emit(this.selectedTags)
  }
}

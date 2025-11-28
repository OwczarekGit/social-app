import {Component, EventEmitter, Input, Output} from '@angular/core';
import {DetailedTag} from "../../../data/detailed-tag";

@Component({
    selector: 'admin-tag-editor-field',
    templateUrl: './admin-tag-editor-field.component.html',
    styleUrls: ['./admin-tag-editor-field.component.css'],
    standalone: false
})
export class AdminTagEditorFieldComponent {

  @Input()
  tag?: DetailedTag

  @Output()
  onTagUpdate = new EventEmitter<DetailedTag>

  @Output()
  onTagDelete = new EventEmitter<DetailedTag>

  update() {
    if (this.tag != null)
      this.onTagUpdate.emit(this.tag)
  }

  delete() {
    if (this.tag != null)
      this.onTagDelete.emit(this.tag)
  }
}

import {Component, EventEmitter, Input, OnChanges, Output, SimpleChanges} from '@angular/core';
import {ListDisplay} from "../../data/list-display";

@Component({
    selector: 'app-list-small',
    templateUrl: './list-small.component.html',
    styleUrls: ['./list-small.component.css'],
    standalone: false
})
export class ListSmallComponent {

  @Output()
  itemClicked: EventEmitter<ListDisplay> = new EventEmitter<ListDisplay>()


  @Input('label')
  label: string = ''

  @Input('items')
  items: ListDisplay[] = []

  selected!: number | null

  selectionSwitched(item: ListDisplay, index: number) {
    this.selected = index
    this.itemClicked.emit(item)
  }
}

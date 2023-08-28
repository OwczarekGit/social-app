import {Component, Input} from '@angular/core';

@Component({
  selector: 'app-list-small',
  templateUrl: './list-small.component.html',
  styleUrls: ['./list-small.component.css']
})
export class ListSmallComponent {
  @Input('labels')
  labels: string[] = []

  @Input('items')
  items: any[] = []
}

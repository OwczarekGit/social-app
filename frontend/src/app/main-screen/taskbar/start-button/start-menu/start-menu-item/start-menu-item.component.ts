import {Component, Input} from '@angular/core';

@Component({
  selector: 'app-start-menu-item',
  templateUrl: './start-menu-item.component.html',
  styleUrls: ['./start-menu-item.component.css']
})
export class StartMenuItemComponent {
  @Input()
  caption!: string

  @Input()
  imageUrl!: string

  @Input()
  hasChildren: boolean = false
}

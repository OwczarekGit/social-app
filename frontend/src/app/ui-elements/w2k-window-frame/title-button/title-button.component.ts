import {Component, Input} from '@angular/core';

@Component({
    selector: 'app-title-button',
    templateUrl: './title-button.component.html',
    styleUrls: ['./title-button.component.css'],
    standalone: false
})
export class TitleButtonComponent {
  @Input('type') type!: 'close' | 'minimize' | 'maximize'
}

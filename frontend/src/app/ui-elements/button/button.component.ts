import {Component, Input} from '@angular/core';

@Component({
    selector: 'app-button',
    templateUrl: './button.component.html',
    styleUrls: ['./button.component.css'],
    standalone: false
})
export class ButtonComponent {
  @Input() caption!: string
  @Input() disabled: boolean = false
}

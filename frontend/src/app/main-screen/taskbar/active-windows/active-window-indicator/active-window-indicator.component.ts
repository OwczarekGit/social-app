import {Component, HostBinding, Input} from '@angular/core';

@Component({
  selector: 'app-active-window-indicator',
  templateUrl: './active-window-indicator.component.html',
  styleUrls: ['./active-window-indicator.component.css']
})
export class ActiveWindowIndicatorComponent {

  @HostBinding('class')
  @Input('class')
  classList: string = ''

  @Input()
  set focused(value: boolean) {
    this._focused = value
    if (value)
      this.classList = 'focused'
    else
      this.classList = ''
  }

  _focused: boolean = false

  @Input('icon')
  icon: string = ''

  @Input('text')
  text: string = ''
}

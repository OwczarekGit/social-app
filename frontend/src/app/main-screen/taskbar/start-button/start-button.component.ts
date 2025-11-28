import {Component, HostBinding, Input} from '@angular/core';

@Component({
    selector: 'app-start-button',
    templateUrl: './start-button.component.html',
    styleUrls: ['./start-button.component.css'],
    host: {
        "(click)": "toggleOpen()"
    },
    standalone: false
})
export class StartButtonComponent {
  @HostBinding('class')
  @Input('class')
  classList: string = ''

  public isOpen: boolean = false

  toggleOpen() {
    this.isOpen = !this.isOpen
    if (!this.isOpen)
      this.classList = ''
    else
      this.classList = 'open'
  }
}

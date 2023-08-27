import {Component, ElementRef, inject, ViewChild} from '@angular/core';
import {WindowComponent} from "../../ui-elements/window/window.component";

@Component({
  selector: 'app-notification',
  templateUrl: './notification.component.html',
  styleUrls: ['./notification.component.css']
})
export class NotificationComponent {

  @ViewChild(WindowComponent)
  window!: WindowComponent

  public title!: string
  public text!: string
  public icon!: string

  public close() {
    this.window.closeWindow()
  }
}

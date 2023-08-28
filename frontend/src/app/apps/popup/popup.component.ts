import {Component, ElementRef, inject, ViewChild} from '@angular/core';
import {WindowComponent} from "../../ui-elements/window/window.component";

@Component({
  selector: 'app-popup',
  templateUrl: './popup.component.html',
  styleUrls: ['./popup.component.css']
})
export class PopupComponent {

  @ViewChild(WindowComponent)
  window!: WindowComponent

  public title!: string
  public text!: string
  public icon!: string

  public close() {
    this.window.closeWindow()
  }
}

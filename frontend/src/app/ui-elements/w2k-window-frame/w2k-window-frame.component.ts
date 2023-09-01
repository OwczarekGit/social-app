import {Component, ElementRef, ViewChild} from '@angular/core';
import {WindowFrame} from "../../data/window-frame";

@Component({
  selector: 'app-w2k-window-frame',
  templateUrl: './w2k-window-frame.component.html',
  styleUrls: ['./w2k-window-frame.component.css'],
  host: {'(mousedown)': 'onFocus()'}
})
export class W2kWindowFrameComponent extends WindowFrame {

  @ViewChild('content')
  content!: ElementRef<HTMLDivElement>

  minimize: boolean = false
  maximize: boolean = false
  close: boolean = true

  onClose!: () => void
  onMinimize!: () => void
  onMaximize!: () => void

  putContent(content: HTMLDivElement): void {
    setTimeout(() => this.content.nativeElement.appendChild(content),0)
  }

  closeClicked() {
    this.onClose()
  }

  minimizeClicked() {
    this.onMinimize()
  }

  maximizeClicked() {
    this.onMaximize()
  }
}

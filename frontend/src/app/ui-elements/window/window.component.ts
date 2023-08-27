import {
  AfterViewInit,
  Component,
  ComponentRef,
  ElementRef, EventEmitter,
  inject,
  Input,
  Output,
  ViewContainerRef
} from '@angular/core';
import {WindowService} from "../../service/window.service";

@Component({
  selector: 'app-window',
  templateUrl: './window.component.html',
  styleUrls: ['./window.component.css'],
  host: {'class': 'w2k-window-border'}
})
export class WindowComponent implements AfterViewInit {

  public id!: number

  @Input()
  windowTitle: string = ""

  @Input()
  windowIconUrl!: string

  @Input()
  minimize: boolean = true

  @Input()
  maximize: boolean = true

  @Input()
  close: boolean = true

  @Output()
  closeClicked = new EventEmitter<any>()

  @Output()
  minimizeClicked = new EventEmitter<any>()

  @Output()
  maximizeClicked = new EventEmitter<any>()

  public windowService = inject(WindowService)
  public host = inject(ViewContainerRef)

  constructor() {
  }

  public closeWindow() {
    this.windowService.close(this.id)
  }

  public setPosition(x: number, y: number) {
    this.windowService.setPosition(this.id, x, y)
  }

  ngAfterViewInit(): void {
    this.windowService.register(this)
  }
}

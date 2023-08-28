import {AfterViewInit, Component, ElementRef, inject, ViewChild, ViewContainerRef} from '@angular/core';
import {WindowService} from "../service/window.service";

@Component({
  selector: 'app-main-screen',
  templateUrl: './main-screen.component.html',
  styleUrls: ['./main-screen.component.css']
})
export class MainScreenComponent implements AfterViewInit {

  @ViewChild('surface', {read: ViewContainerRef})
  surface!: ViewContainerRef

  private windowService = inject(WindowService)

  ngAfterViewInit(): void {
    this.windowService.setSurface(this.surface)
  }

}

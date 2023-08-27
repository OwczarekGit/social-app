import {AfterViewInit, Component, inject, ViewContainerRef} from '@angular/core';
import {WindowService} from "../service/window.service";

@Component({
  selector: 'app-main-screen',
  templateUrl: './main-screen.component.html',
  styleUrls: ['./main-screen.component.css']
})
export class MainScreenComponent implements AfterViewInit {

  private windowService = inject(WindowService)
  private vcr = inject(ViewContainerRef)

  ngAfterViewInit(): void {
    this.windowService.setSurface(this.vcr)
  }

}

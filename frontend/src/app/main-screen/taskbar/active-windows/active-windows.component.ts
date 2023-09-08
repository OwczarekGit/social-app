import {Component, inject} from '@angular/core';
import {WindowService} from "../../../service/window.service";

@Component({
  selector: 'app-active-windows',
  templateUrl: './active-windows.component.html',
  styleUrls: ['./active-windows.component.css']
})
export class ActiveWindowsComponent {
  public windowService = inject(WindowService)

  indicatorClicked(id: number) {
    this.windowService.focusApplication(id)
  }

}

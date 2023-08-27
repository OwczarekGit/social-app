import {AfterViewInit, Component, inject, ViewContainerRef} from '@angular/core';
import {WindowService} from "../service/window.service";
import {RegistrationFormComponent} from "../forms/registration-form/registration-form.component";
import {NotificationComponent} from "../apps/notification/notification.component";

@Component({
  selector: 'app-display',
  templateUrl: './display.component.html',
  styleUrls: ['./display.component.css']
})
export class DisplayComponent implements AfterViewInit {

  public vcr = inject(ViewContainerRef)
  private windowService = inject(WindowService)

  ngAfterViewInit(): void {
    this.windowService.setSurface(this.vcr)
    setTimeout(() => this.windowService.openApplication(RegistrationFormComponent), 0)
  }

}

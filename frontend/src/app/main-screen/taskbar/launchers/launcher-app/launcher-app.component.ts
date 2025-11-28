import {Component, Input} from '@angular/core';

@Component({
    selector: 'app-launcher-app',
    templateUrl: './launcher-app.component.html',
    styleUrls: ['./launcher-app.component.css'],
    standalone: false
})
export class LauncherAppComponent {
  @Input()
  icon: string = ""
}

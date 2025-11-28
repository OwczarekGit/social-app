import {AfterViewInit, Component} from '@angular/core';

@Component({
    selector: 'app-clock',
    templateUrl: './clock.component.html',
    styleUrls: ['./clock.component.css'],
    standalone: false
})
export class ClockComponent implements AfterViewInit {
  time: Date = new Date()

  ngAfterViewInit(): void {
    setInterval(() => {
      this.time = new Date()
    },1000)
  }
}

import {AfterViewInit, Component} from '@angular/core';

@Component({
  selector: 'app-clock',
  templateUrl: './clock.component.html',
  styleUrls: ['./clock.component.css']
})
export class ClockComponent implements AfterViewInit {
  time: string = ""

  ngAfterViewInit(): void {
    setInterval(() => {
      let now = new Date()
      let hour = now.getHours()
      let min = now.getMinutes()

      this.time = `${hour}:${min}`
    },1000)
  }
}

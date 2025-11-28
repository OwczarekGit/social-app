import {AfterViewInit, Component, ElementRef, inject} from '@angular/core';
import {WallpaperService} from "./service/wallpaper.service";
import {Some} from "option-value";

@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.css'],
    standalone: false
})
export class AppComponent implements AfterViewInit {
  title = 'frontend';

  host = inject(ElementRef)
  wallpaperService = inject(WallpaperService)

  ngAfterViewInit(): void {
    this.wallpaperService.backgroundElement = Some(this.host.nativeElement as HTMLDivElement)
  }
}

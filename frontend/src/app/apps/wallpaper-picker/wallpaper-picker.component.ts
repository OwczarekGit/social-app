import {AfterViewInit, Component, inject, signal} from '@angular/core';
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {ImageService} from "../../service/image.service";
import {Wallpaper} from "../../data/wallpaper";
import {ListDisplay} from "../../data/list-display";
import {WallpaperService} from "../../service/wallpaper.service";

@Component({
  selector: 'app-wallpaper-picker',
  templateUrl: './wallpaper-picker.component.html',
  styleUrls: ['./wallpaper-picker.component.css']
})
export class WallpaperPickerComponent extends WindowContent<null, W2kWindowFrameComponent> implements AfterViewInit {

  public wallpaperService = inject(WallpaperService)

  wallpapers = signal<Wallpaper[]>([])
  selectedWallpaper = signal<Wallpaper | null>(null)

  constructor() {
    super();
    this.wallpaperService.getAllWallpapers().subscribe({
      next: value => this.wallpapers.set(value.map(v => new Wallpaper(v.id, v.title, v.url)))
    })
  }

  ngAfterViewInit(): void {
    setTimeout(() => {
      this.setTitle("Wallpaper Settings")
      this.setIcon("/assets/wallpaper-settings-s.png")
    })
    this.windowFrame.onClose = () => this.closeWindow()
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)
  }

  wallpaperClicked(wallpaper: ListDisplay) {
    this.selectedWallpaper.set(wallpaper as Wallpaper)
  }

  setWallpaper() {
    if (this.selectedWallpaper() == null) return
    this.wallpaperService.setWallpaper(this.selectedWallpaper()?.id as number).subscribe({
      next: value => {
        this.wallpaperService.restoreWallpaper()
      }
    })
  }

  resetWallpaper() {
    this.wallpaperService.resetWallpaper()
  }
}

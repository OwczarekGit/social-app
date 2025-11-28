import {Component, ElementRef, inject, signal, ViewChild} from '@angular/core';
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";
import {ProfileService} from "../../service/profile.service";
import {PopupService} from "../../service/popup.service";
import {Profile} from "../../data/profile";

@Component({
    selector: 'app-change-profile-picture',
    templateUrl: './change-profile-picture.component.html',
    styleUrls: ['./change-profile-picture.component.css'],
    standalone: false
})
export class ChangeProfilePictureComponent extends WindowContent<null, W2kWindowFrameComponent>{

  private profileService = inject(ProfileService)
  private popupService = inject(PopupService)

  @ViewChild('picker')
  picker!: ElementRef<HTMLInputElement>

  @ViewChild('preview')
  preview!: ElementRef<HTMLImageElement>

  public file = signal<File | null>(null)

  ngAfterViewInit(): void {
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)
    this.windowFrame.onClose = () => this.closeWindow()
    setTimeout(() => {
      this.setTitle("Change profile picture")
      this.setIcon("/assets/profile-picture-s.png")
    })

    this.profileService.getMyProfile().subscribe({
      next: v => {
        this.preview.nativeElement.src = new Profile(v.user_id, v.username, v.picture_url).picture_url
      }
    })
  }

  setSelectedImage() {
    if (this.picker == null || this.picker.nativeElement.files == null) return
    let file = this.picker.nativeElement.files.item(0)
    if (file == null) return;

    let reader = new FileReader()
    reader.onload = (e) => {
      // @ts-ignore
      this.preview.nativeElement.src = e.target.result
    }

    reader.readAsDataURL(file)
    this.file.set(file)
  }

  updateProfilePicture() {
    let file = this.file()
    if (file == null) return

    this.profileService.setProfilePicture(file).subscribe({
      next: _ => {
        this.popupService.info(
          "Profile picture updated",
          "The profile picture has been successfully updated."
        )
        this.closeWindow()
      },
      error: _ => {
        this.popupService.info(
          "Error",
          "There was an error while updating profile picture. Was the provided file a valid image?"
        )
      }
    })
  }
}

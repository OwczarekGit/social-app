import {AfterViewInit, Component, inject } from '@angular/core';
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {ProfileService} from "../../service/profile.service";
import {PopupService} from "../../service/popup.service";
import {WindowContent} from "../../data/window-content";
import {W2kWindowFrameComponent} from "../../ui-elements/w2k-window-frame/w2k-window-frame.component";

@Component({
    selector: 'app-change-username',
    templateUrl: './change-username.component.html',
    styleUrls: ['./change-username.component.css'],
    standalone: false
})
export class ChangeUsernameComponent extends WindowContent<null, W2kWindowFrameComponent> implements AfterViewInit {
  public form = new FormGroup({
    username: new FormControl<string>('', Validators.required)
  })

  private profileService = inject(ProfileService)
  private popupService = inject(PopupService)

  constructor() {
    super()
    this.profileService.getMyProfile().subscribe({
      next: value => this.form.controls.username.setValue(value.username)
    })

  }

  changeUsername() {
    let form = this.form.getRawValue()

    this.profileService.changeUsername(form.username as string).subscribe({
      complete: () => {
        this.popupService.info("Username changed", `Your username has been changed to ${form.username}.`)
        this.closeWindow()
      }
    })
  }

  ngAfterViewInit(): void {
    this.windowFrame.onFocus = () => this.wm.focusApplication(this.id)
    this.windowFrame.onClose = () => this.wm.closeApplication(this.id)

    setTimeout(() => {
      this.setTitle("Change username")
      this.setIcon("/assets/profile-username.png")
    })
  }
}

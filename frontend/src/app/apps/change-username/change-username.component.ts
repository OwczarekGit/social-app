import {AfterViewInit, Component, inject, ViewChild} from '@angular/core';
import {WindowComponent} from "../../ui-elements/window/window.component";
import {FormControl, FormGroup, Validators} from "@angular/forms";
import {ProfileService} from "../../service/profile.service";
import {PopupService} from "../../service/popup.service";

@Component({
  selector: 'app-change-username',
  templateUrl: './change-username.component.html',
  styleUrls: ['./change-username.component.css']
})
export class ChangeUsernameComponent {
  public form = new FormGroup({
    username: new FormControl<string>('', Validators.required)
  })

  private profileService = inject(ProfileService)
  private popupService = inject(PopupService)

  @ViewChild(WindowComponent)
  window!: WindowComponent

  constructor() {
    this.profileService.getMyProfile().subscribe({
      next: value => this.form.controls.username.setValue(value.username)
    })
  }

  public close() {
    this.window.closeWindow()
  }

  changeUsername() {
    let form = this.form.getRawValue()

    this.profileService.changeUsername(form.username as string).subscribe({
      complete: () => {
        this.popupService.info("Username changed", `Your username has been changed to ${form.username}.`)
        this.close()
      }
    })
  }
}

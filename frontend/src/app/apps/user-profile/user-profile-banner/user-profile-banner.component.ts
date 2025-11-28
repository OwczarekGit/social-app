import {Component, Input} from '@angular/core';
import {Profile} from "../../../data/profile";

@Component({
    selector: 'app-user-profile-banner',
    templateUrl: './user-profile-banner.component.html',
    styleUrls: ['./user-profile-banner.component.css'],
    standalone: false
})
export class UserProfileBannerComponent {
  @Input('profile')
  profile!: Profile | null
}

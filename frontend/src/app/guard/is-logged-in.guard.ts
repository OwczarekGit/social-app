import { CanActivateFn } from '@angular/router';
import {inject} from "@angular/core";
import {AuthService} from "../service/auth.service";

export const isLoggedInGuard: CanActivateFn = (route, state) => {
  let auth = inject(AuthService)

  return auth.isLoggedIn()
};

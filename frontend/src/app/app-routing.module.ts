import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import {LoginScreenComponent} from "./login-screen/login-screen.component";
import {MainScreenComponent} from "./main-screen/main-screen.component";
import {isLoggedInGuard} from "./guard/is-logged-in.guard";

const routes: Routes = [
  {
    path: "",
    component: LoginScreenComponent
  },
  {
    path: "desktop",
    component: MainScreenComponent,
    canActivate: [isLoggedInGuard]
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }

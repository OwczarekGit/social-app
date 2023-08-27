import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { InputComponent } from './ui-elements/input/input.component';
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import { RegistrationFormComponent } from './forms/registration-form/registration-form.component';
import { ButtonComponent } from './ui-elements/button/button.component';
import { WindowComponent } from './ui-elements/window/window.component';
import { TitleButtonComponent } from './ui-elements/window/title-button/title-button.component';
import { LoginScreenComponent } from './login-screen/login-screen.component';
import {HttpClientModule} from "@angular/common/http";
import { NotificationComponent } from './apps/notification/notification.component';
import { LoginFormComponent } from './forms/login-form/login-form.component';
import { MainScreenComponent } from './main-screen/main-screen.component';
import { TaskbarComponent } from './main-screen/taskbar/taskbar.component';
import { StartButtonComponent } from './main-screen/taskbar/start-button/start-button.component';
import { SystemTrayComponent } from './main-screen/taskbar/system-tray/system-tray.component';
import { ClockComponent } from './main-screen/taskbar/system-tray/clock/clock.component';
import { DraggableDirective } from './directives/draggable.directive';
import { StartMenuComponent } from './main-screen/taskbar/start-button/start-menu/start-menu.component';
import { StartMenuItemComponent } from './main-screen/taskbar/start-button/start-menu/start-menu-item/start-menu-item.component';

@NgModule({
  declarations: [
    AppComponent,
    InputComponent,
    RegistrationFormComponent,
    ButtonComponent,
    WindowComponent,
    TitleButtonComponent,
    LoginScreenComponent,
    NotificationComponent,
    LoginFormComponent,
    MainScreenComponent,
    TaskbarComponent,
    StartButtonComponent,
    SystemTrayComponent,
    ClockComponent,
    DraggableDirective,
    StartMenuComponent,
    StartMenuItemComponent,
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    FormsModule,
    ReactiveFormsModule,
    HttpClientModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }

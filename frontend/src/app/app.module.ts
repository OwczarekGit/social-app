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
import { DisplayComponent } from './display/display.component';
import {HttpClientModule} from "@angular/common/http";
import { NotificationComponent } from './apps/notification/notification.component';

@NgModule({
  declarations: [
    AppComponent,
    InputComponent,
    RegistrationFormComponent,
    ButtonComponent,
    WindowComponent,
    TitleButtonComponent,
    DisplayComponent,
    NotificationComponent,
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

import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { InputComponent } from './ui-elements/input/input.component';
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import { RegistrationFormComponent } from './forms/registration-form/registration-form.component';
import { ButtonComponent } from './ui-elements/button/button.component';
import { TitleButtonComponent } from './ui-elements/w2k-window-frame/title-button/title-button.component';
import { LoginScreenComponent } from './login-screen/login-screen.component';
import {HttpClientModule} from "@angular/common/http";
import { PopupComponent } from './apps/popup/popup.component';
import { LoginFormComponent } from './forms/login-form/login-form.component';
import { MainScreenComponent } from './main-screen/main-screen.component';
import { TaskbarComponent } from './main-screen/taskbar/taskbar.component';
import { StartButtonComponent } from './main-screen/taskbar/start-button/start-button.component';
import { SystemTrayComponent } from './main-screen/taskbar/system-tray/system-tray.component';
import { ClockComponent } from './main-screen/taskbar/system-tray/clock/clock.component';
import { DraggableDirective } from './directives/draggable.directive';
import { StartMenuComponent } from './main-screen/taskbar/start-button/start-menu/start-menu.component';
import { StartMenuItemComponent } from './main-screen/taskbar/start-button/start-menu/start-menu-item/start-menu-item.component';
import { PostWriterComponent } from './apps/post-writer/post-writer.component';
import { TextareaComponent } from './ui-elements/textarea/textarea.component';
import { FriendManagerComponent } from './apps/friend-manager/friend-manager.component';
import { ListSmallComponent } from './ui-elements/list-small/list-small.component';
import { PeopleSearcherComponent } from './apps/people-searcher/people-searcher.component';
import { NotificationIndicatorComponent } from './main-screen/taskbar/system-tray/notification-indicator/notification-indicator.component';
import { NotificationCenterComponent } from './apps/notification-center/notification-center.component';
import { NotificationEntryComponent } from './apps/notification-center/notification-entry/notification-entry.component';
import { ChangeUsernameComponent } from './apps/change-username/change-username.component';
import { ShareImageComponent } from './apps/share-image/share-image.component';
import { TagPickerComponent } from './apps/tag-picker/tag-picker.component';
import { ButtonIconComponent } from './ui-elements/button-icon/button-icon.component';
import { LaunchersComponent } from './main-screen/taskbar/launchers/launchers.component';
import { LauncherAppComponent } from './main-screen/taskbar/launchers/launcher-app/launcher-app.component';
import { AdminTagEditorComponent } from './admin/admin-tag-editor/admin-tag-editor.component';
import { AdminTagEditorFieldComponent } from './admin/admin-tag-editor/admin-tag-editor-field/admin-tag-editor-field.component';
import { W2kWindowFrameComponent } from './ui-elements/w2k-window-frame/w2k-window-frame.component';
import { ActiveWindowsComponent } from './main-screen/taskbar/active-windows/active-windows.component';
import { ActiveWindowIndicatorComponent } from './main-screen/taskbar/active-windows/active-window-indicator/active-window-indicator.component';

@NgModule({
  declarations: [
    AppComponent,
    InputComponent,
    RegistrationFormComponent,
    ButtonComponent,
    TitleButtonComponent,
    LoginScreenComponent,
    PopupComponent,
    LoginFormComponent,
    MainScreenComponent,
    TaskbarComponent,
    StartButtonComponent,
    SystemTrayComponent,
    ClockComponent,
    DraggableDirective,
    StartMenuComponent,
    StartMenuItemComponent,
    PostWriterComponent,
    TextareaComponent,
    FriendManagerComponent,
    ListSmallComponent,
    PeopleSearcherComponent,
    NotificationIndicatorComponent,
    NotificationCenterComponent,
    NotificationEntryComponent,
    ChangeUsernameComponent,
    ShareImageComponent,
    TagPickerComponent,
    ButtonIconComponent,
    LaunchersComponent,
    LauncherAppComponent,
    AdminTagEditorComponent,
    AdminTagEditorFieldComponent,
    W2kWindowFrameComponent,
    ActiveWindowsComponent,
    ActiveWindowIndicatorComponent,
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

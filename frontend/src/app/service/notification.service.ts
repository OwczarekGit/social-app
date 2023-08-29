import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {NotificationType} from "../const/notification-type";
import {SoundService} from "./sound.service";

@Injectable({
  providedIn: 'root'
})
export class NotificationService {

  private es!: EventSource
  private http = inject(HttpClient)
  private soundService = inject(SoundService)

  constructor() {
  }

  public subscribeToNotifications() {
    this.es = new EventSource("/api/notification/subscribe")
    this.es.addEventListener("message", (ev) => {
      const notification = JSON.parse(ev.data)
      const notification_type = notification["notification_type"] as NotificationType
      this.soundService.notification()

      switch (notification_type) {
        case NotificationType.MESSAGE: {
          const payload = JSON.parse(notification["data"])
          console.log(payload)
        } break;
        case NotificationType.FRIEND_REQUEST: {
          console.log(notification['data'])
        } break;
      }
    })
  }
}

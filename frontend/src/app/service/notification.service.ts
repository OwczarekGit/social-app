import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {NotificationType} from "../const/notification-type";

@Injectable({
  providedIn: 'root'
})
export class NotificationService {

  private es!: EventSource
  private http = inject(HttpClient)

  constructor() {
    this.subscribeToNotifications()
  }

  public subscribeToNotifications() {
    this.es = new EventSource("/api/notification/subscribe")
    this.es.addEventListener("message", (ev) => {
      const notification = JSON.parse(ev.data)
      const type = notification["notification_type"] as NotificationType

      switch (type) {
        case NotificationType.MESSAGE: {
          const payload = JSON.parse(notification["data"])
          console.log(payload)
        } break;
      }
    })
  }
}

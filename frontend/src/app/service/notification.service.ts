import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Notification} from "../data/notification";
import {SoundService} from "./sound.service";
import {Subject} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class NotificationService {

  private es!: EventSource
  private http = inject(HttpClient)
  private soundService = inject(SoundService)
  public onNewNotification: Subject<Notification> = new Subject<Notification>()

  public notifications: Notification[] = []

  constructor() {
  }

  public dismissNotification(id: number) {
    return this.http.delete("/api/notification/"+id).subscribe({
      complete:() => {
        this.notifications.splice(this.notifications.findIndex(n => n.id == id),1)
      }
    })
  }

  public subscribeToNotifications() {
    this.es = new EventSource("/api/notification/subscribe")
    this.es.addEventListener("message", n => {
      let json: Notification = JSON.parse(n.data)
      this.soundService.message()
      this.notifications.unshift(this.parseNotification(json))
      this.onNewNotification.next(this.parseNotification(json))
    })
  }

  public loadRemainingNotifications() {
    this.http.get<Notification[]>("/api/notification").subscribe({
      next: value => {
        this.notifications = value.map(v => this.parseNotification(v))
      }
    })
  }

  private parseNotification(data: Notification): Notification {
    let newJson = JSON.parse(data.notification_data as any)

    return {
      id: data.id,
      date: data.date,
      notification_data: newJson
    }
  }
}

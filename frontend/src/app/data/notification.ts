import {NotificationType} from "../const/notification-type";

export interface Notification {
  id: number
  date: Date,
  notification_data: NotificationData
}

export interface NotificationData {
  notification_type: NotificationType
  data: string
}

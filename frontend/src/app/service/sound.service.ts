import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class SoundService {

  constructor() { }

  public error() {
    let a = document.createElement('audio')
    a.src = "./assets/error.mp3"
    a.play()
  }

  public message() {
    let a = document.createElement('audio')
    a.src = "./assets/message.mp3"
    a.play()
  }

  public notification() {
    let a = document.createElement('audio')
    a.src = "./assets/notification.mp3"
    a.play()
  }

  public login() {
    let a = document.createElement('audio')
    a.src = "./assets/login.mp3"
    a.volume = .5
    a.play()
  }
}

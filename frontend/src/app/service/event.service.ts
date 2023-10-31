import { Injectable } from '@angular/core';
import {Subject} from "rxjs";
import {Post} from "../data/post";

@Injectable({
  providedIn: 'root'
})
export class EventService {

  public postEditedSub: Subject<Post> = new Subject<Post>()
  public postDeletedSub: Subject<Post> = new Subject<Post>()
  public postCreatedSub: Subject<Post> = new Subject<Post>()

  public emitPostEdited(post: Post) {
    this.notify(this.postEditedSub, post)
  }

  public emitPostDeleted(post: Post) {
    this.notify(this.postDeletedSub, post)
  }

  public emitPostCreated(post: Post) {
    this.notify(this.postCreatedSub, post)
  }

  private notify<T>(sub: Subject<T>, item: T) {
    sub.next(item)
  }

  constructor() { }
}

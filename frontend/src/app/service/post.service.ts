import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class PostService {

  private http = inject(HttpClient)

  constructor() { }

  public writePost(content: string): Observable<any> {
    return this.http.post("/api/post/create", {
      content: content
    })
  }
}

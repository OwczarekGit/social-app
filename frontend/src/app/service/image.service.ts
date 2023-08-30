import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class ImageService {

  private http = inject(HttpClient)

  constructor() { }

  public uploadImage(title: string, tags: string[], image: File): Observable<any> {
    let fd = new FormData()
    fd.append("title", title)
    tags.forEach(t => fd.append("tags", t))
    fd.append("image", image)

    return this.http.post("/api/image", fd)
  }
}

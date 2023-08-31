import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {DetailedTag} from "../data/detailed-tag";

@Injectable({
  providedIn: 'root'
})
export class TagService {

  private http = inject(HttpClient)

  constructor() { }

  public getAllDetailedTags(): Observable<DetailedTag[]> {
    return this.http.get<DetailedTag[]>("/api/tag")
  }

  public updateTag(id: number, name: string): Observable<any> {
    return this.http.put("/api/admin/tag/"+id, {
      name: name
    })
  }

  public deleteTag(id: number): Observable<any> {
    return this.http.delete("/api/admin/tag/"+id)
  }

  public createNewTag(name: string): Observable<DetailedTag> {
    return this.http.post<DetailedTag>("/api/admin/tag", {
      name: name
    })
  }
}

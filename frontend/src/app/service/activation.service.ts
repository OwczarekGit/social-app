import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {ActivationEmailTemplate} from "../data/activation-email-template";

@Injectable({
  providedIn: 'root'
})
export class ActivationService {

  private http = inject(HttpClient)

  constructor() { }

  public getActivationEmailTemplate(): Observable<ActivationEmailTemplate | null> {
    return this.http.get<ActivationEmailTemplate | null>("/api/admin/activation")
  }

  public updateActivationEmailTemplate(value: string): Observable<any> {
    return this.http.put("/api/admin/activation", {
      content: value
    })
  }
}

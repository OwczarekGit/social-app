import {inject, Injectable} from '@angular/core';
import {Observable} from "rxjs";
import {HttpClient} from "@angular/common/http";
import {VariableResponse} from "../data/variable-response";

@Injectable({
  providedIn: 'root'
})
export class DomainService {
  private http = inject(HttpClient)

  private _systemDomain: string = ''
  private _imageDomain: string = ''

  get systemDomain(): string {
    return this._systemDomain
  }

  get imageDomain(): string {
    return this._imageDomain
  }

  constructor() {
    this.refreshDomains()
  }

  public getSystemDomain(): Observable<VariableResponse | null> {
    return this.http.get<VariableResponse | null>("/api/domain/system")
  }

  public getImageDomain(): Observable<VariableResponse | null> {
    return this.http.get<VariableResponse | null>("/api/domain/image")
  }

  public setImageDomain(value: string): Observable<any> {
    return this.http.put("/api/admin/domain/image", {
      value: value
    })
  }

  public setSystemDomain(value: string): Observable<any> {
    return this.http.put("/api/admin/domain/system", {
      value: value
    })
  }

  refreshDomains() {
    this.getSystemDomain().subscribe({
      next: v => {
        if (v != null)
          this._systemDomain = v.value
      }
    })

    this.getImageDomain().subscribe({
      next: v => {
        if (v != null)
          this._imageDomain = v.value
      }
    })
  }
}

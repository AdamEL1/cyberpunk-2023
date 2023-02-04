import { Injectable } from '@angular/core';
import { HttpClient } from "@angular/common/http";
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class CommunicationService {
  private readonly BASE_URL: string = '';

  constructor(private readonly http: HttpClient) { }

  async get<T>(route: string){
    return await this.observableToPromise(this.http.get(`${this.BASE_URL}/${route}`));
  }

  async post<T, R>(body: T, route: string){
    return await this.observableToPromise<R>(this.http.post<R>(`${this.BASE_URL}/${route}`, body));
  }

  private async observableToPromise<T>(observable: Observable<T>): Promise<T> {
    return new Promise<T>((resolve, reject) => {
      observable.subscribe({
        next: (value: T) => resolve(value),
        error: (error) => reject(error)
      });
    });
  }

}

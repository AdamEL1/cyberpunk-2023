import { Injectable } from '@angular/core';
import { HttpClient } from "@angular/common/http";
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class CommunicationService {
  private readonly BASE_URL: string = 'localhost:8080';

  constructor(private readonly http: HttpClient) { }

  async get<T>(route: string){
    const endPoint: string = `${this.BASE_URL}/${route}`;
    console.log(`GET, at: ${endPoint}`);
    return await this.observableToPromise(this.http.get(endPoint));
  }

  async post<T, R>(body: T, route: string){
    const endPoint: string = `${this.BASE_URL}/${route}`;
    console.log(`POST, at: ${endPoint}`);
    return await this.observableToPromise<R>(this.http.post<R>(endPoint, body));
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

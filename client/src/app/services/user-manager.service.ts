import { Injectable } from '@angular/core';
import { User } from '../classes/user';

@Injectable({
  providedIn: 'root'
})
export class UserManagerService {
  private connectedUser: User | null = null;

  constructor() {
    this.connectedUser = {
      name: "adam", 
      courses: [{title: "INF3710"}, {title: "second random"}],
      Description: "ANY",
      email: "holla@gmail.com",
      school: "poly"
    };
   }

  connectUser(newUser: User): void {
    this.connectedUser = newUser;
  }

  disconnectUser(): void {
    this.connectedUser = null;
  }

  // getters
  getUser(): User | null {
    return this.connectedUser;
  }
}

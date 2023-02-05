import { Injectable } from '@angular/core';
import { getMockedDescription } from '../classes/description';
import { User } from '../classes/user';

@Injectable({
  providedIn: 'root'
})
export class UserManagerService {
  public suggestedUsers: User[] = []; 

  private connectedUser: User | null = null;

  constructor() {
    this.connectedUser = {
      name: "adam", 
      password: "salut",
      courses: [{title: "INF3710"}, {title: "second random"}],
      description: getMockedDescription(),
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

  clearSuggestedUsers(): void {
    this.suggestedUsers = [];
  }

  // getters
  getUser(): User | null {
    return this.connectedUser;
  }
}

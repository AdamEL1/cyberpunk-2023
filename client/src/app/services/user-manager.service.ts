import { Injectable } from '@angular/core';
import { getMockedDescription } from '../classes/description';
import { SuggestedUser } from '../classes/suggestedUser';
import { User } from '../classes/user';

@Injectable({
  providedIn: 'root'
})
export class UserManagerService {
  public suggestedUsers: SuggestedUser[] = []; 

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

    this.suggestedUsers = [
      {user: this.connectedUser, weight: 1},
      {user: this.connectedUser, weight: 2},
      {user: this.connectedUser, weight: 3},
      {user: this.connectedUser, weight: 4},
      {user: this.connectedUser, weight: 5},
    ];
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

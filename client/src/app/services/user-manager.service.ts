import { Injectable } from '@angular/core';
import { SuggestedUser } from '../classes/suggestedUser';
import { User } from '../classes/user';

@Injectable({
  providedIn: 'root'
})
export class UserManagerService {
  public suggestedUsers: SuggestedUser[] = []; 

  private connectedUser: User | null = null;

  constructor() {}

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

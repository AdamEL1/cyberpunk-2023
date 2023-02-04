import { Injectable } from '@angular/core';
import { User } from '../classes/user';

@Injectable({
  providedIn: 'root'
})
export class UserManagerService {
  private connectedUser: User | null = null;

  constructor() { }

  connectUser(newUser: User): void {
    this.connectedUser = newUser;
  }

  disconnectUser(): void {
    this.connectedUser = null;
  }
}

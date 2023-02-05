import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { User } from 'src/app/classes/user';
import { UserManagerService } from 'src/app/services/user-manager.service';


@Component({
  selector: 'app-suggested-users',
  templateUrl: './suggested-users.component.html',
  styleUrls: ['./suggested-users.component.css']
})
export class SuggestedUsersComponent {
  public email: string = '';
  constructor(public userManager: UserManagerService, private router: Router) { 
    if(!this.userManager.getUser()) router.navigateByUrl("");
  }

  sendEmail(selectedUser: User){
    this.email = "mailto:" + selectedUser.email;
  }
}


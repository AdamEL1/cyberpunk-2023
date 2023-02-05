import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { UserManagerService } from 'src/app/services/user-manager.service';

@Component({
  selector: 'app-suggested-users',
  templateUrl: './suggested-users.component.html',
  styleUrls: ['./suggested-users.component.css']
})
export class SuggestedUsersComponent {

  constructor(public userManager: UserManagerService, private router: Router) { 
    if(!this.userManager.getUser()) router.navigateByUrl("");
  }

}

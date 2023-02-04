import { Component, OnInit } from '@angular/core';
import { UserManagerService } from 'src/app/services/user-manager.service';

@Component({
  selector: 'app-user',
  templateUrl: './user.component.html',
  styleUrls: ['./user.component.css']
})
export class UserComponent implements OnInit {

  constructor(public userManager: UserManagerService) { }

  ngOnInit(): void {

  }

}

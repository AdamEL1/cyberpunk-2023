import { Component, OnInit } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { UserManagerService } from 'src/app/services/user-manager.service';

@Component({
  selector: 'app-user',
  templateUrl: './user.component.html',
  styleUrls: ['./user.component.css']
})
export class UserComponent implements OnInit {

  constructor(public userManager: UserManagerService, public dialog:MatDialog) { }

  ngOnInit(): void {
  }

  openDialog(): void{
  }

}

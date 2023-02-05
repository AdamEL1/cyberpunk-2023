import { Component, OnInit } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { CourseDialogComponent } from 'src/app/components/course-dialog/course-dialog.component';
import { UserManagerService } from 'src/app/services/user-manager.service';

@Component({
  selector: 'app-user',
  templateUrl: './user.component.html',
  styleUrls: ['./user.component.css']
})
export class UserComponent {

  constructor(public userManager: UserManagerService, public dialog:MatDialog) { }

  openDialog(): void{
    const dialogRef = this.dialog.open(CourseDialogComponent);
  }

  async getSuggestedUsers(courseTitle: string){
    console.log(courseTitle);
  }

}

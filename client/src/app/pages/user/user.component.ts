import { Component, OnInit } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { Router } from '@angular/router';
import { Course } from 'src/app/classes/course';
import { SuggestedUser } from 'src/app/classes/suggestedUser';
import { User } from 'src/app/classes/user';
import { CourseDialogComponent } from 'src/app/components/course-dialog/course-dialog.component';
import { SUGGESTION_USERS_ROUTE } from 'src/app/constants';
import { CommunicationService } from 'src/app/services/communication.service';
import { UserManagerService } from 'src/app/services/user-manager.service';

@Component({
  selector: 'app-user',
  templateUrl: './user.component.html',
  styleUrls: ['./user.component.css']
})
export class UserComponent {

  constructor(public userManager: UserManagerService, public dialog:MatDialog, private router: Router, private communicationService: CommunicationService) { }

  openDialog(): void{
    const dialogRef = this.dialog.open(CourseDialogComponent);
  }

  async getSuggestedUsers(course: Course){
    console.log(course);
    type weihgtedSuggestions = {data: SuggestedUser[]};
    type CourseSuggestionData = {name: string, password: string, course: Course};
    const courseSuggestionData: CourseSuggestionData = {
      name: this.userManager.getUser()!.name,
      password: this.userManager.getUser()!.password,
      course: course
    };
    // request
    const suggestionData: weihgtedSuggestions = await this.communicationService.post<CourseSuggestionData, weihgtedSuggestions>(courseSuggestionData, SUGGESTION_USERS_ROUTE);
    // save data in user-management
    this.userManager.suggestedUsers = suggestionData.data;
    this.router.navigateByUrl('suggested-users');
  }

}

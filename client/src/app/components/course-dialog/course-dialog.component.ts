import { Component, OnInit } from '@angular/core';
import { FormControl } from '@angular/forms';
import { MatDialog, MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { Course } from 'src/app/classes/course';
import { ADD_COURSE_ROUTE } from 'src/app/constants';
import { CommunicationService } from 'src/app/services/communication.service';
import { UserManagerService } from 'src/app/services/user-manager.service';
@Component({
  selector: 'app-course-dialog',
  templateUrl: './course-dialog.component.html',
  styleUrls: ['./course-dialog.component.css']
})
export class CourseDialogComponent {
  public courseInput: FormControl;
  
  constructor(private communicationService: CommunicationService, private userManager: UserManagerService, private dialogRef: MatDialogRef<CourseDialogComponent>) {
    this.courseInput = new FormControl('');
  }

  async addCourse(){
    type CourseJoinData = {name: string, password: string, course: Course};
    const course: Course = {
      title: this.courseInput.value
    };
    const joinData: CourseJoinData = {
      name: this.userManager.getUser()!.name,
      password: this.userManager.getUser()!.password,
      course: course
    };
    console.log(joinData);
    const state: {state: boolean} = await this.communicationService.post<CourseJoinData, {state: boolean}>(joinData, ADD_COURSE_ROUTE);
    if(!state.state) return;
    this.userManager.getUser()!.courses.push(course);
    this.courseInput.reset();
    this.dialogRef.close();
  }

  isValid(): boolean{
    return this.courseInput.valid;
  }

}

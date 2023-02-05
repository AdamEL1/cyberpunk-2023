import { Component, OnInit } from '@angular/core';
import { FormControl } from '@angular/forms';
import { MatDialog } from '@angular/material/dialog';
import { Router } from '@angular/router';
import { Course } from 'src/app/classes/course';
import { User } from 'src/app/classes/user';
import { SliderDialogueComponent } from 'src/app/components/slider-dialogue/slider-dialogue.component';
import { REGISTER_USER_ROUTE } from 'src/app/constants';
import { CommunicationService } from 'src/app/services/communication.service';
import { UserManagerService } from 'src/app/services/user-manager.service';

const NUM_FIELD_FORM = 5;

export enum FILED_INDEX {
  NAME_FIELD_INDEX = 0,
  PASSWORD_FIELD_INDEX = 1,
  SCHOOL_FIELD_INDEX = 2,
  EMAIL_FIELD_INDEX = 3,
  COURSES_FIELD_INDEX = 4
};

@Component({
  selector: 'app-register',
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.css']
})
export class RegisterComponent {
  public form: FormControl[] = [];
  private attributeOpened = false;
  private description: number[] = [];

  constructor(private communicationService: CommunicationService, private router: Router, private userManager: UserManagerService, private dialog: MatDialog ) { 
    for(let i = 0; i < NUM_FIELD_FORM; i++) this.form[i] = new FormControl("");
  }

  async register() {
    const user: User = {
      name: this.form[FILED_INDEX.NAME_FIELD_INDEX].value,
      password: this.form[FILED_INDEX.PASSWORD_FIELD_INDEX].value,
      email: this.form[FILED_INDEX.EMAIL_FIELD_INDEX].value,
      school: this.form[FILED_INDEX.SCHOOL_FIELD_INDEX].value,
      courses: (this.form[FILED_INDEX.COURSES_FIELD_INDEX].value as string).split(',').map((value: string): Course => {
        return {title: value};
      }),
      description: this.description,
    };
    const val = await this.communicationService.post<User, {state: boolean}>(user, REGISTER_USER_ROUTE);
    // TODO, to implement handling getting an error;
    this.userManager.connectUser(user);
    this.clearForm();
    this.router.navigateByUrl('/user');
  }

  async openAttributeBox(){
    const ref = this.dialog.open(SliderDialogueComponent);
    ref.afterClosed().subscribe((passed: {data: number[]}) => {
      this.description = passed.data;
      this.attributeOpened = true;
    })
  }

  get isFormValid(): boolean{
    if(!this.attributeOpened) return false;
    for(const formControl of this.form) if(!formControl.valid) return false;
    return true;
  }

  private clearForm(): void {
    for(let i = 0; i < NUM_FIELD_FORM; i++) this.form[i].reset();
  }

}

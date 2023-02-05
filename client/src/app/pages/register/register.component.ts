import { Component, OnInit } from '@angular/core';
import { FormControl } from '@angular/forms';
import { Course } from 'src/app/classes/course';
import { User } from 'src/app/classes/user';
import { CommunicationService } from 'src/app/services/communication.service';

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
export class RegisterComponent implements OnInit {
  public form: FormControl[] = [];

  constructor(private communicationService: CommunicationService) { 
    for(let i = 0; i < NUM_FIELD_FORM; i++) this.form[i] = new FormControl("");
  }

  ngOnInit(): void {

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
      Description: ''
    };
    console.log(user);
    await this.communicationService.post();
    this.clearForm();
  }

  private clearForm(): void {
    for(let i = 0; i < NUM_FIELD_FORM; i++) this.form[i].reset();
  }

}

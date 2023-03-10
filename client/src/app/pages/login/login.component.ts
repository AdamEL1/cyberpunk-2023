import { Component, OnInit } from '@angular/core';
import { FormControl } from '@angular/forms';
import { Router } from '@angular/router';
import { Course } from 'src/app/classes/course';
import { User } from 'src/app/classes/user';
import { LOGIN_ROUTE } from 'src/app/constants';
import { CommunicationService } from 'src/app/services/communication.service';
import { UserManagerService } from 'src/app/services/user-manager.service';

const NUM_FIELD_FORM = 2;

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css']
})
export class LoginComponent {
  public form: FormControl[] = [];
  public loginState: boolean = true;

  constructor(private communicationService: CommunicationService, private userManger: UserManagerService, private router: Router) {
    this.form[0] = new FormControl("");
    this.form[1] = new FormControl("");
  }

  async login() {
    let user;
    try{
      user = await this.communicationService.post<{name: string, password: string}, User>({
        name: this.form[0].value,
        password: this.form[1].value,
      }, LOGIN_ROUTE);
      user.courses = (user.courses as unknown as string[]).map((title: string): Course => {
        return { title: title };
      })
    }catch{
      console.log("catched");
      this.loginState = false;
      return;
    }
    if(this.isUserValid(user)){
      user.password = this.form[1].value;
      console.log(user);
      this.userManger.connectUser(user);
      this.loginState = true;
      this.clearForm();
      this.router.navigateByUrl('/user');
      return;
    }
    this.loginState = false;
  }

  get isFormValid(): boolean{
    for(const formControl of this.form) if(!formControl.valid) return true;
    return false;
  }

  private isUserValid(user: User){
    return user.name != '';
  }

  private clearForm(): void {
    for(let i = 0; i < NUM_FIELD_FORM; i++) this.form[i].reset();
  }

}

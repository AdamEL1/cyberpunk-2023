import { Component, OnInit } from '@angular/core';
import { FormControl } from '@angular/forms';
import { LOGIN_ROUTE } from 'src/app/constants';
import { CommunicationService } from 'src/app/services/communication.service';

const NUM_FIELD_FORM = 2;

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css']
})
export class LoginComponent implements OnInit {
  public form: FormControl[] = [];

  constructor(private communicationService: CommunicationService) {
    this.form[0] = new FormControl("");
    this.form[1] = new FormControl("");
  }

  ngOnInit(): void {

  }

  async login() {
    this.communicationService.post<{username: string, password: string}, User>({
      username: this.form[0].value,
      password: this.form[1].value,
    }, LOGIN_ROUTE);
    this.clearForm();
  }

  private clearForm(): void {
    for(let i = 0; i < NUM_FIELD_FORM; i++) this.form[i].reset();
  }

}

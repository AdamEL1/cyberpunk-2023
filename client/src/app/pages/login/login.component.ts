import { Component, OnInit } from '@angular/core';
import { FormControl } from '@angular/forms';
import { CommunicationService } from 'src/app/services/communication.service';

const NUM_FIELD_FORM = 2;

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css']
})
export class LoginComponent implements OnInit {
  public form: FormControl[] = [];

  constructor(communicationService: CommunicationService) {
    this.form[0] = new FormControl("");
    this.form[1] = new FormControl("");
  }

  ngOnInit(): void {

  }

  async login() {
    console.log(this.form[0].value);
    console.log(this.form[1].value);
    this.clearForm();
  }

  private clearForm(): void {
    for(let i = 0; i < NUM_FIELD_FORM; i++) this.form[i].reset();
  }

}

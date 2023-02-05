import { Component, OnInit } from '@angular/core';
import { FormControl } from '@angular/forms';

const NUM_FIELD_FORM = 3;

@Component({
  selector: 'app-register',
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.css']
})
export class RegisterComponent implements OnInit {
  public form: FormControl[] = [];

  constructor() { 
    this.form[0] = new FormControl("");
    this.form[1] = new FormControl("");
    this.form[2] = new FormControl("");
  }

  ngOnInit(): void {
  }

  async register() {
    console.log(this.form[0].value);
    console.log(this.form[1].value);
    this.clearForm();
  }

  private clearForm(): void {
    for(let i = 0; i < NUM_FIELD_FORM; i++) this.form[i].reset();
  }





}

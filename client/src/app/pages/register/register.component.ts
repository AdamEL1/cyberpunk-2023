import { Component, OnInit } from '@angular/core';
import { FormControl } from '@angular/forms';

const NUM_FIELD_FORM = 4;

@Component({
  selector: 'app-register',
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.css']
})
export class RegisterComponent implements OnInit {
  public form: FormControl[] = [];

  constructor() { 
    for(let i = 0; i < NUM_FIELD_FORM; i++) this.form[i] = new FormControl("");
  }

  ngOnInit(): void {
    
  }

  async register() {
    for(let i = 0; i < NUM_FIELD_FORM; i++) console.log(this.form[i].value);
    this.clearForm();
  }

  private clearForm(): void {
    for(let i = 0; i < NUM_FIELD_FORM; i++) this.form[i].reset();
  }

}

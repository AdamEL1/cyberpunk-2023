import { Component } from '@angular/core';
import { FormControl } from '@angular/forms';
import { MatDialogRef } from '@angular/material/dialog';

const NUM_FIELDS = 6;

@Component({
  selector: 'app-slider-dialogue',
  templateUrl: './slider-dialogue.component.html',
  styleUrls: ['./slider-dialogue.component.css']
})
export class SliderDialogueComponent {
  public formControls: FormControl[] = [];

  constructor(private dialogueRef: MatDialogRef<SliderDialogueComponent>) { 
    for(let i = 0; i < NUM_FIELDS; i++) this.formControls[i] = new FormControl(5);
  }

  submitDialog(){
    const values: number[] = [];
    for(let i = 0; i < NUM_FIELDS; i++) values.push(this.formControls[i].value);
    this.dialogueRef.close({data: values});
  }

}

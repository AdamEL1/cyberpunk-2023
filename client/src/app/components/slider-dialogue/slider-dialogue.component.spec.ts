import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SliderDialogueComponent } from './slider-dialogue.component';

describe('SliderDialogueComponent', () => {
  let component: SliderDialogueComponent;
  let fixture: ComponentFixture<SliderDialogueComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ SliderDialogueComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SliderDialogueComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

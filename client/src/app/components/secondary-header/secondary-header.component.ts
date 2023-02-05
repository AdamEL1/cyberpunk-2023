import { Component, OnInit } from '@angular/core';
import { UserManagerService } from 'src/app/services/user-manager.service';

@Component({
  selector: 'app-secondary-header',
  templateUrl: './secondary-header.component.html',
  styleUrls: ['./secondary-header.component.css']
})
export class SecondaryHeaderComponent implements OnInit {

  constructor(public userManager: UserManagerService) { }

  ngOnInit(): void {
  }

}

import { Component, Input } from '@angular/core';
import { ProgressBarMode } from '@angular/material/progress-bar';

@Component({
  selector: 'app-progress-bar',
  templateUrl: './progress-bar.component.html',
  styleUrls: ['./progress-bar.component.scss']
})
export class ProgressBarComponent {
  @Input() public current: number;
  @Input() public max: number;
  @Input() public mode: ProgressBarMode;
  public Math = Math;

  constructor() { }

}

import { Component, OnInit, Inject } from '@angular/core';
import { MAT_SNACK_BAR_DATA } from '@angular/material/snack-bar';

import {
  faBomb,
  faExclamationTriangle,
  faInfoCircle,
  IconDefinition
} from '@fortawesome/free-solid-svg-icons';

export enum NotificationLevel {
  Error,
  Warn,
  Info
}

@Component({
  selector: 'app-icon-snack-bar',
  templateUrl: './icon-snackbar.component.html',
  styleUrls: ['./icon-snackbar.component.scss']
})
export class IconSnackBarComponent implements OnInit {
  icon: IconDefinition = faBomb;

  constructor(@Inject(MAT_SNACK_BAR_DATA) public data: any) { }

  ngOnInit(): void {
    const icons = {
      [NotificationLevel.Error]: faBomb,
      [NotificationLevel.Warn]: faExclamationTriangle,
      [NotificationLevel.Info]: faInfoCircle
    }
    this.icon = icons[this.data.level];
  }

}

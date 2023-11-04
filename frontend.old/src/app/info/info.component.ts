import { Component} from '@angular/core';

import { Exchanges } from '../rpc/entities_pb';

@Component({
  selector: 'app-info',
  templateUrl: './info.component.html',
  styleUrls: ['./info.component.scss']
})
export class InfoComponent {
  public exchanges: string[];

  constructor() {
    this.exchanges = Object
      .keys(Exchanges)
      .map(
        (upperName) => upperName[0].toUpperCase() +
          upperName.substr(1).toLowerCase()
      );
  }
}

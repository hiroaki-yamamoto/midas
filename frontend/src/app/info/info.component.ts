import { Component } from '@angular/core';

import { Exchanges } from '../../rpc/exchanges.zod';

@Component({
  selector: 'app-info',
  templateUrl: './info.component.html',
  styleUrls: ['./info.component.scss']
})
export class InfoComponent {
  public exchanges = Exchanges.options;
}

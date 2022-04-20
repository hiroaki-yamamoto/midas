import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, ParamMap } from '@angular/router';
import { Exchanges } from '../rpc/entities_pb';

import { faRotate } from '@fortawesome/free-solid-svg-icons';

@Component({
  selector: 'app-sync',
  templateUrl: './sync.component.html',
  styleUrls: ['./sync.component.scss']
})
export class SyncComponent implements OnInit {

  public exchange: Exchanges;
  public rotateIcon = faRotate;
  public alreadySynced: boolean = false;

  constructor(private curRoute: ActivatedRoute) {}

  ngOnInit(): void {
    this.curRoute.paramMap.subscribe((params: ParamMap) => {
      this.exchange = parseInt(params.get('exchange'), 10) as Exchanges;
    });
  }

}

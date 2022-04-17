import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Exchanges } from './rpc/entities_pb';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  public readonly exchanges = Object.values(Exchanges);
  constructor(private http: HttpClient) {}

  preventCSRF() {
    this.http.head('/token/csrf').subscribe();
  }
}

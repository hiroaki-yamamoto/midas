import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Exchanges } from '../rpc/exchanges.zod';

@Component({
  standalone: false,
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  public readonly exchanges = Exchanges.options;
  constructor(private http: HttpClient) { }

  preventCSRF() {
    this.http.head('/token/csrf').subscribe();
  }
}

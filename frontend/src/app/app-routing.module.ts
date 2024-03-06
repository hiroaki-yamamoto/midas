import { NgModule, inject } from '@angular/core';
import {
  Routes, RouterModule,
  ActivatedRouteSnapshot,
} from '@angular/router';
import { Observable } from 'rxjs';

import { DashboardComponent } from './dashboard/dashboard.component';
import { InfoComponent } from './info/info.component';
import { KeychainComponent } from './keychain/keychain.component';
import { BotEditorComponent } from './bot-editor/bot-editor.component';
import {
  ActivationService as BotEditorActivationService
} from './bot-editor/activation.service';
import { SyncComponent } from './sync/sync.component';


const routes: Routes = [
  {
    path: 'info',
    component: InfoComponent,
  },
  {
    path: 'api',
    component: KeychainComponent
  },
  {
    path: 'edit-bot/:botId',
    component: BotEditorComponent,
    canActivate: [
      (route: ActivatedRouteSnapshot): Observable<boolean> => {
        return inject(BotEditorActivationService).canActivate(route);
      }
    ],
  },
  {
    path: 'sync/:exchange',
    component: SyncComponent,
  },
  {
    path: '',
    component: DashboardComponent,
  },
  {
    path: '**',
    redirectTo: ''
  }
];

@NgModule({
  imports: [
    RouterModule.forRoot(routes)
  ],
  exports: [RouterModule]
})
export class AppRoutingModule { }

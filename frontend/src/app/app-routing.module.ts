import { NgModule } from '@angular/core';
import {
  Routes, RouterModule,
  ActivatedRouteSnapshot, RouterStateSnapshot,
} from '@angular/router';
import { DashboardComponent } from './dashboard/dashboard.component';
import { InfoComponent } from './info/info.component';
import { KeychainComponent } from './keychain/keychain.component';
import { BotEditorComponent } from './bot-editor/bot-editor.component';
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
      (route: ActivatedRouteSnapshot, state: RouterStateSnapshot): boolean => {
        const botId = route.params.botId;
        if (!botId) {
          return true;
        }
        throw new Error('Not implemented');
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

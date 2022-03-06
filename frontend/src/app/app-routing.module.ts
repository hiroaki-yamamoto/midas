import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { DashboardComponent } from './dashboard/dashboard.component';
import { InfoComponent } from './info/info.component';
import { KeychainComponent } from './keychain/keychain.component';
import { BotEditorComponent } from './bot-editor/bot-editor.component';
import { SyncSymbolComponent } from './sync-symbol/sync-symbol.component';
import { SyncHistoryComponent } from './sync-history/sync-history.component';


const routes: Routes = [
  {
    path: '',
    component: DashboardComponent,
  },
  {
    path: 'info',
    component: InfoComponent,
  },
  {
    path: 'api',
    component: KeychainComponent
  },
  {
    path: 'edit-bot',
    component: BotEditorComponent,
  },
  {
    path: 'sync-symbol',
    component: SyncSymbolComponent,
  },
  {
    path: 'sync-history',
    component: SyncHistoryComponent,
  }
];

@NgModule({
  imports: [
    RouterModule.forRoot(routes)
  ],
  exports: [RouterModule]
})
export class AppRoutingModule { }

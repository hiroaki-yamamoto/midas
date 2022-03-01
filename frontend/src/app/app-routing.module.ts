import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { DashboardComponent } from './dashboard/dashboard.component';
import { InfoComponent } from './info/info.component';
import { KeychainComponent } from './keychain/keychain.component';
import { BotEditorComponent } from './bot-editor/bot-editor.component';
import { SyncComponent } from './sync/sync.component';


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
    path: 'sync',
    component: SyncComponent,
  }
];

@NgModule({
  imports: [
    RouterModule.forRoot(routes)
  ],
  exports: [RouterModule]
})
export class AppRoutingModule { }

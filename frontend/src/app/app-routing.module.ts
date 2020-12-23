import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { DashboardComponent } from './dashboard/dashboard.component';
import { DashboardModule } from './dashboard/dashboard.module';
import { InfoComponent } from './info/info.component';
import { KeychainComponent } from './keychain/keychain.component';


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
  }
];

@NgModule({
  imports: [
    DashboardModule,
    RouterModule.forRoot(routes)
  ],
  exports: [RouterModule]
})
export class AppRoutingModule { }

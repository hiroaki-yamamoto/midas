import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FlexLayoutModule } from '@angular/flex-layout';
import { MatExpansionModule } from '@angular/material/expansion';

import { DashboardComponent } from './dashboard.component';
import { BotPanelComponent } from './bot-panel/bot-panel.component';
import { MatListModule } from '@angular/material/list';

@NgModule({
  declarations: [DashboardComponent, BotPanelComponent],
  imports: [
    CommonModule,
    FlexLayoutModule,
    MatExpansionModule,
    MatListModule,
  ],
  exports: [
    DashboardComponent,
  ]
})
export class DashboardModule {
}

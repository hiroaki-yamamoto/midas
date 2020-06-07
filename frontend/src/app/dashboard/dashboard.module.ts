import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FlexLayoutModule } from '@angular/flex-layout';
import { MatExpansionModule } from '@angular/material/expansion';

import { DashboardComponent } from './dashboard.component';
import { BotPanelComponent } from './bot-panel/bot-panel.component';

@NgModule({
  declarations: [DashboardComponent, BotPanelComponent],
  imports: [
    CommonModule,
    FlexLayoutModule,
    MatExpansionModule,
  ],
  exports: [
    DashboardComponent,
  ]
})
export class DashboardModule {
}

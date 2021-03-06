import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FlexLayoutModule } from '@angular/flex-layout';
import { MatExpansionModule } from '@angular/material/expansion';

import { DashboardComponent } from './dashboard.component';
import { BotPanelComponent } from './bot-panel/bot-panel.component';
import { MatTableModule } from '@angular/material/table';
import { MatTabsModule } from '@angular/material/tabs';
import { MatPaginatorModule } from '@angular/material/paginator';

@NgModule({
  declarations: [DashboardComponent, BotPanelComponent],
  imports: [
    CommonModule,
    FlexLayoutModule,
    MatExpansionModule,
    MatPaginatorModule,
    MatTableModule,
    MatTabsModule,
  ],
  exports: [
    DashboardComponent,
  ]
})
export class DashboardModule {
}

import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { FlexLayoutModule } from '@angular/flex-layout';
import { HttpClientModule, HttpClientXsrfModule } from '@angular/common/http';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';

import { MatCardModule } from '@angular/material/card'
import { MatCheckboxModule } from '@angular/material/checkbox';
import { MatDialogModule } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatButtonModule } from '@angular/material/button';
import { MatToolbarModule } from '@angular/material/toolbar';
import {
  MatSnackBarModule,
  MAT_SNACK_BAR_DEFAULT_OPTIONS
} from '@angular/material/snack-bar';
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { MatTabsModule } from '@angular/material/tabs';
import { MatPaginatorModule } from '@angular/material/paginator';
import { MatTableModule } from '@angular/material/table';
import { MatSortModule } from '@angular/material/sort';
import { MatIconModule } from '@angular/material/icon';
import { MatListModule } from '@angular/material/list';
import { MatInputModule } from '@angular/material/input';
import { MatTooltipModule } from '@angular/material/tooltip';
import { MatSelectModule } from '@angular/material/select';
import { MatExpansionModule } from '@angular/material/expansion';
import { MatMenuModule } from '@angular/material/menu';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';

import { MonacoEditorModule } from 'ngx-monaco-editor'

import { BotPanelComponent } from './bot-panel/bot-panel.component';
import { DashboardComponent } from './dashboard/dashboard.component';
import { IconSnackBarComponent } from './icon-snackbar/icon-snackbar.component';
import { InfoComponent } from './info/info.component';
import { TradeObserverService } from './resources/trade-observer.service';
import { KeychainService } from './resources/keychain.service';
import { BookTickerComponent } from './info/book-ticker/book-ticker.component';
import { KeychainComponent } from './keychain/keychain.component';
import { EditDialogComponent } from './keychain/edit-dialog/edit-dialog.component';
import { ExchangePipePipe } from './rpc/exchange-pipe.pipe';
import { DeleteWarnComponent } from './keychain/delete-warn/delete-warn.component';
import { BotEditorComponent } from './bot-editor/bot-editor.component';
import { DateGraphComponent } from './date-graph/date-graph.component';
import { SyncComponent } from './sync/sync.component';
import { ProgressBarComponent } from './progress-bar/progress-bar.component';

@NgModule({
  declarations: [
    AppComponent,
    IconSnackBarComponent,
    InfoComponent,
    BookTickerComponent,
    KeychainComponent,
    EditDialogComponent,
    ExchangePipePipe,
    DeleteWarnComponent,
    BotPanelComponent,
    BotEditorComponent,
    DashboardComponent,
    DateGraphComponent,
    SyncComponent,
    ProgressBarComponent,
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    HttpClientXsrfModule,
    HttpClientModule,

    MonacoEditorModule.forRoot(),

    ReactiveFormsModule,
    FlexLayoutModule,
    MatCardModule,
    MatCheckboxModule,
    MatDialogModule,
    MatExpansionModule,
    MatFormFieldModule,
    MatInputModule,
    MatButtonModule,
    MatPaginatorModule,
    MatProgressBarModule,
    MatSelectModule,
    MatSnackBarModule,
    MatTableModule,
    MatTabsModule,
    MatToolbarModule,
    MatTooltipModule,
    MatSortModule,
    MatIconModule,
    MatListModule,
    MatMenuModule,
    FontAwesomeModule,
  ],
  providers: [
    { provide: MAT_SNACK_BAR_DEFAULT_OPTIONS, useValue: { duration: 5000 } }
  ],
  bootstrap: [AppComponent]
})
export class AppModule {
  constructor(
    tradeObserver: TradeObserverService,
    keychain: KeychainService,
  ) {
    tradeObserver.connect();
    keychain.fetch().subscribe();
  }
}

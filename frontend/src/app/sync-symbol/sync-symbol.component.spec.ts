import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SyncSymbolComponent } from './sync-symbol.component';

describe('SyncSymbolComponent', () => {
  let component: SyncSymbolComponent;
  let fixture: ComponentFixture<SyncSymbolComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ SyncSymbolComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(SyncSymbolComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

import { ComponentFixture, TestBed } from '@angular/core/testing';

import { IconSnackBarComponent } from './icon-snackbar.component';

describe('IconSnackBarComponent', () => {
  let component: IconSnackBarComponent;
  let fixture: ComponentFixture<IconSnackBarComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ IconSnackBarComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(IconSnackBarComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

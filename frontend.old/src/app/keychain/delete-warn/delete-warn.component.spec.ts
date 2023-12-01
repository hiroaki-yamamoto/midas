import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DeleteWarnComponent } from './delete-warn.component';

describe('DeleteWarnComponent', () => {
  let component: DeleteWarnComponent;
  let fixture: ComponentFixture<DeleteWarnComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ DeleteWarnComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(DeleteWarnComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

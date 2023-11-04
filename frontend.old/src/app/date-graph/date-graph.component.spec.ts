import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DateGraphComponent } from './date-graph.component';

describe('DateGraphComponent', () => {
  let component: DateGraphComponent;
  let fixture: ComponentFixture<DateGraphComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ DateGraphComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(DateGraphComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BookTickerComponent } from './book-ticker.component';

describe('BookTickerComponent', () => {
  let component: BookTickerComponent;
  let fixture: ComponentFixture<BookTickerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ BookTickerComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(BookTickerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

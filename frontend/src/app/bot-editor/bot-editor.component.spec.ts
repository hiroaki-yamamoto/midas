import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BotEditorComponent } from './bot-editor.component';

describe('BotEditorComponent', () => {
  let component: BotEditorComponent;
  let fixture: ComponentFixture<BotEditorComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ BotEditorComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(BotEditorComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

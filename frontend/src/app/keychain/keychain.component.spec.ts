import { ComponentFixture, TestBed } from '@angular/core/testing';

import { KeychainComponent } from './keychain.component';

describe('KeychainComponent', () => {
  let component: KeychainComponent;
  let fixture: ComponentFixture<KeychainComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ KeychainComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(KeychainComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

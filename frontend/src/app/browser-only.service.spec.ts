import { TestBed } from '@angular/core/testing';

import { BrowserOnlyService } from './browser-only.service';

describe('BrowserOnlyService', () => {
  let service: BrowserOnlyService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(BrowserOnlyService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});

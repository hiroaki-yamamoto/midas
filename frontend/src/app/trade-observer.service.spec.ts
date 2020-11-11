import { TestBed } from '@angular/core/testing';

import { TradeObserverService } from './trade-observer.service';

describe('TradeObserverService', () => {
  let service: TradeObserverService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(TradeObserverService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});

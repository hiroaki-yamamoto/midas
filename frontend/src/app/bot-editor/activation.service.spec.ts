import { TestBed } from '@angular/core/testing';

import { ActivationService } from './activation.service';

describe('ActivationServiceService', () => {
  let service: ActivationService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ActivationService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});

import { inject } from '@angular/core';
import { Router } from '@angular/router';
import { PassportService } from '../_services/passport-service';

export const authGuard = () => {
  const passportService = inject(PassportService);
  const router = inject(Router);

  if (passportService.data()) {
    return true;
  } else {
    router.navigate(['/login']);
    return false;
  }
};
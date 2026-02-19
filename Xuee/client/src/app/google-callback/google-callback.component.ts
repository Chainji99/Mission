import { Component, OnInit, inject } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { PassportService } from '../_services/passport-service';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';

@Component({
  selector: 'app-google-callback',
  standalone: true,
  imports: [MatProgressSpinnerModule],
  template: `
    <div style="display: flex; justify-content: center; align-items: center; height: 100vh; flex-direction: column; gap: 20px;">
      <mat-spinner></mat-spinner>
      <p>Logging in with Google...</p>
      @if (error) {
        <p style="color: red">{{ error }}</p>
      }
    </div>
  `
})
export class GoogleCallbackComponent implements OnInit {
  private route = inject(ActivatedRoute);
  private router = inject(Router);
  private passport = inject(PassportService);
  
  error = '';

  ngOnInit() {
    this.route.queryParams.subscribe(async params => {
      const code = params['code'];
      if (code) {
        const err = await this.passport.loginWithGoogle(code);
        if (err) {
          this.error = err;
        } else {
          this.router.navigate(['/']);
        }
      } else {
        this.error = 'No code provided';
        setTimeout(() => this.router.navigate(['/login']), 2000);
      }
    });
  }
}

import { Component, computed, inject, Signal } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatMenuModule } from '@angular/material/menu';
import { MatToolbarModule } from '@angular/material/toolbar';
import { Router } from '@angular/router';
import { PassportService } from '../_services/passport-service';  

@Component({
  selector: 'app-navbar',
  imports: [MatToolbarModule, MatButtonModule, MatMenuModule],
  templateUrl: './navbar.html',
  styleUrl: './navbar.scss',    
})
export class Navbar {
 private _Passport = inject(PassportService)
 display_name: Signal<string | undefined>
 avatar_url: Signal<string | undefined>

 constructor() {
  this.display_name = computed(() => this._Passport.data()?.display_name);
  this.avatar_url = computed(() => this._Passport.data()?.avatar_url);
}

  logout() {
    this._Passport.logout();
    const router = inject(Router);
    router.navigate(['/login']);
  }
}

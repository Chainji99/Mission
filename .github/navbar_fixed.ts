import { Component, computed, inject, Signal } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatMenuModule } from '@angular/material/menu';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatDialog } from '@angular/material/dialog';
import { Router } from '@angular/router';
import { PassportService } from '../_services/passport-service';  
import { UploadImgDialog } from '../upload-img/upload-img';  

@Component({
  selector: 'app-navbar',
  imports: [MatToolbarModule, MatButtonModule, MatMenuModule],
  templateUrl: './navbar.html',
  styleUrl: './navbar.scss',    
})
export class Navbar {
 private _Passport = inject(PassportService)
 private _dialog = inject(MatDialog);
 private _router = inject(Router);
 display_name: Signal<string | undefined>
 avatar_url: Signal<string | undefined>

 constructor() {
  this.display_name = computed(() => this._Passport.data()?.display_name);
  this.avatar_url = computed(() => this._Passport.data()?.avatar_url);
}

  logout() {
    this._Passport.logout();
    this._router.navigate(['/login']);
  }

  login() {
    this._router.navigate(['/login']);
  }

  onUploadAvatar() {
    this._dialog.open(UploadImgDialog);
  }
}

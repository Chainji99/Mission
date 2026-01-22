import { Component } from '@angular/core';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatButtonModule } from '@angular/material/button';
import { MatMenuModule } from '@angular/material/menu';
import { signal } from '@angular/core';

@Component({
  selector: 'app-navbar',
  templateUrl: './navbar.html',
  styleUrls: ['./navbar.scss'],
  standalone: true, // ← ต้องมี!
  imports: [ // ← ระบุโมดูลที่ใช้งาน
    MatToolbarModule,
    MatButtonModule,
    MatMenuModule,
    // ... ฯลฯ
  ]
})
export class NavbarComponent {
  display_name = signal('');
  avatar_url = signal('');

  logout() {
  
  }
}
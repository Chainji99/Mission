import { Component, computed, inject, Signal, ChangeDetectorRef } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatMenuModule } from '@angular/material/menu';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatIconModule } from '@angular/material/icon';
import { MatDialog, MatDialogModule } from '@angular/material/dialog';
import { CommonModule } from '@angular/common';
import { Router, RouterLink, RouterLinkActive } from '@angular/router';
import { PassportService } from '../_services/passport-service';
import { Profile } from '../profile/profile';
import { ChatService } from '../_services/chat-service';
import { ChatPanel } from '../chat-panel/chat-panel';
import { FriendPanel } from '../friend-panel/friend-panel';

@Component({
  selector: 'app-navbar',
  standalone: true,
  imports: [CommonModule, MatToolbarModule, MatButtonModule, MatMenuModule, MatIconModule, MatDialogModule, RouterLink, RouterLinkActive, ChatPanel, FriendPanel],
  templateUrl: './navbar.html',
  styleUrls: ['./navbar.scss'],
})
export class Navbar {
  private _Passport = inject(PassportService)
  private _dialog = inject(MatDialog);
  private _cdr = inject(ChangeDetectorRef);
  private _chat = inject(ChatService);
  private _router = inject(Router);
  display_name: Signal<string | undefined>
  avatar_url: Signal<string | undefined>
  chatRooms: Array<{ id: number, name: string }> = [];

  constructor() {
    this.display_name = computed(() => this._Passport.data()?.display_name);
    this.avatar_url = computed(() => this._Passport.data()?.avatar_url);
    this._chat.rooms$.subscribe(r => { this.chatRooms = r; this._cdr.detectChanges(); });
  }

  logout() {
    this._Passport.destroy();
    this._router.navigate(['/login']);
  }

  openProfile() {
    console.log('Attempting to open profile dialog...');
    try {
      const dialogRef = this._dialog.open(Profile, {
        width: '800px',
        maxWidth: '95vw',
        panelClass: 'discord-profile-dialog',
        autoFocus: false,
        restoreFocus: true
      });

      dialogRef.afterOpened().subscribe(() => {
        console.log('Profile dialog opened successfully');
        this._cdr.detectChanges();
      });

      dialogRef.afterClosed().subscribe(result => {
        console.log('Profile dialog closed:', result);
      });
    } catch (error) {
      console.error('Error opening profile dialog:', error);
    }
  }

  openChat(id: number) {
    console.log('Navbar.openChat called for', id);
    this._router.navigate([`/mission-chat/${id}`]);
  }
}

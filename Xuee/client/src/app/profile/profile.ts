import { Component, computed, inject, Signal, signal, Optional } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatDialog, MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatIconModule } from '@angular/material/icon';
import { getAvatarUrl } from '../_helpers/util';
import { PassportService } from '../_services/passport-service';
import { UploadImgDialog } from '../upload-img/upload-img';

@Component({
  selector: 'app-profile',
  standalone: true,
  imports: [
    CommonModule, 
    FormsModule, 
    MatDialogModule, 
    MatButtonModule, 
    MatInputModule, 
    MatFormFieldModule,
    MatIconModule
  ],
  templateUrl: './profile.html',
  styleUrls: ['./profile.scss'],
})
export class Profile {
  private _passport = inject(PassportService);
  private _dialog = inject(MatDialog);
  private _dialogRef = inject(MatDialogRef<Profile>, { optional: true });

  passportData = computed(() => this._passport.data());
  avatar_url = computed(() => getAvatarUrl(this.passportData()));
  
  memberSince = computed(() => {
    const data = this.passportData();
    const dateStr = data?.created_at;
    
    // If we have no date but we are signed in, use today as a fallback for the display
    if (!dateStr) {
      if (this._passport.isSignin()) {
        return new Date().toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
      }
      return 'Unknown';
    }
    
    const date = new Date(dateStr);
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
  });
  
  // Display Name editing
  isEditingName = signal(false);
  newName = signal('');
  
  // Username editing
  isEditingUsername = signal(false);
  newUsername = signal('');
  
  // Email editing
  isEditingEmail = signal(false);
  newEmail = signal('');
  emailVisible = signal(false);

  isSaving = signal(false);

  constructor() {
    this.resetFields();
  }

  resetFields(): void {
    const data = this.passportData();
    this.newName.set(data?.display_name || '');
    this.newUsername.set(data?.username || '');
    this.newEmail.set(data?.email || '');
  }

  onEditAvatar(): void {
    const dialogRef = this._dialog.open(UploadImgDialog);
    dialogRef.afterClosed().subscribe(result => {
      if (result === true) {
        // Avatar already updated in PassportService
      }
    });
  }

  // Edit methods for each field
  startEdit(field: 'name' | 'username' | 'email'): void {
    this.resetFields();
    if (field === 'name') this.isEditingName.set(true);
    if (field === 'username') this.isEditingUsername.set(true);
    if (field === 'email') this.isEditingEmail.set(true);
  }

  async save(field: 'name' | 'username' | 'email'): Promise<void> {
    if (this.isSaving()) return;
    
    this.isSaving.set(true);
    try {
      if (field === 'name') {
        await this._passport.updateDisplayName(this.newName().trim());
        this.isEditingName.set(false);
      } else if (field === 'username') {
        await this._passport.updateUsername(this.newUsername().trim());
        this.isEditingUsername.set(false);
      } else if (field === 'email') {
        await this._passport.updateEmail(this.newEmail().trim());
        this.isEditingEmail.set(false);
      }
    } catch (error) {
      console.error(`Save ${field} failed`, error);
    } finally {
      this.isSaving.set(false);
    }
  }

  cancelEdit(field: 'name' | 'username' | 'email'): void {
    if (field === 'name') this.isEditingName.set(false);
    if (field === 'username') this.isEditingUsername.set(false);
    if (field === 'email') this.isEditingEmail.set(false);
    this.resetFields();
  }

  toggleEmailVisibility(): void {
    this.emailVisible.update(v => !v);
  }

  getMaskedEmail(email: string | undefined): string {
    if (!email) return '';
    if (this.emailVisible()) return email;
    const [user, domain] = email.split('@');
    return '*'.repeat(user.length) + '@' + domain;
  }

  close(): void {
    if (this._dialogRef) {
      this._dialogRef.close();
    }
  }
}

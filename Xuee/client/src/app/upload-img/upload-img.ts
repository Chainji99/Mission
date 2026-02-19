import { Component, inject, signal } from '@angular/core';
import { MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';
import { UserService } from '../_services/user-service';
import { fileToBase64 } from '../_helpers/file';

@Component({
  selector: 'app-upload-img',
  standalone: true,
  imports: [
    MatDialogModule,
    MatButtonModule,
    MatFormFieldModule,
    MatInputModule,
    MatIconModule,
    MatProgressSpinnerModule,
    FormsModule,
    CommonModule,
  ],
  templateUrl: './upload-img.html',
  styleUrl: './upload-img.scss',
})
export class UploadImgDialog {
  private _userService = inject(UserService);
  private _dialogRef = inject(MatDialogRef<UploadImgDialog>);

  selectedFile: File | null = null;
  previewUrl: string | null = null;
  isUploading = signal(false);
  errorMessage = signal<string | null>(null);

  onFileSelected(event: Event) {
    const input = event.target as HTMLInputElement;
    if (input.files && input.files[0]) {
      const file = input.files[0];
      
      // Validate file type
      const allowedTypes = ['image/png', 'image/jpeg', 'image/jpg', 'image/gif'];
      if (!allowedTypes.includes(file.type)) {
        this.errorMessage.set('รองรับเฉพาะไฟล์รูปภาพ PNG, JPG และ GIF เท่านั้น');
        this.selectedFile = null;
        this.previewUrl = null;
        return;
      }

      // Validate file size (e.g., max 5MB)
      if (file.size > 5 * 1024 * 1024) {
        this.errorMessage.set('ขนาดไฟล์ต้องไม่เกิน 5MB');
        this.selectedFile = null;
        this.previewUrl = null;
        return;
      }

      this.selectedFile = file;
      this.errorMessage.set(null);
      this.generatePreview();
    }
  }

  private async generatePreview() {
    if (this.selectedFile) {
      this.previewUrl = await fileToBase64(this.selectedFile);
    }
  }

  async onUpload() {
    if (this.selectedFile) {
      this.isUploading.set(true);
      this.errorMessage.set(null);
      try {
        const result = await this._userService.uploadAvatar(this.selectedFile);
        if (result && result.includes('successfully')) {
          this._dialogRef.close(true);
        } else {
          this.errorMessage.set(result || 'อัปโหลดล้มเหลว');
        }
      } catch (error) {
        console.error('Upload failed', error);
        this.errorMessage.set('เกิดข้อผิดพลาดระหว่างการอัปโหลด');
      } finally {
        this.isUploading.set(false);
      }
    }
  }

  onCancel() {
    this._dialogRef.close();
  }
}
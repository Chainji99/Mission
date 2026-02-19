import { ChangeDetectionStrategy, Component, inject, signal } from '@angular/core';
import { MatDialogRef, MatDialogActions, MatDialogContent, MatDialogTitle } from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { UserService } from '../_services/user-service'

@Component({
  selector: 'app-upload-img-dialog',
  templateUrl: './upload-img.html',
  styleUrls: ['./upload-img.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [MatDialogActions, MatDialogContent, MatDialogTitle, MatButtonModule]
})
export class UploadImgDialog {
  acceptedMimetype = ['image/png', 'image/jpeg', 'image/jpg', 'image/gif'];
  imgfile: File | undefined;
  imgPreview = signal<string | undefined>(undefined);
  errorMsg = signal<string | undefined>(undefined);
  isUploading = signal<boolean>(false);
  private readonly _dialogRef = inject(MatDialogRef<UploadImgDialog>);
  private readonly _userService = inject(UserService);

  onSubmit(): void {
    if (this.imgfile) {
      this.isUploading.set(true);
      this._userService.uploadAvatar(this.imgfile).then(result => {
        this.isUploading.set(false);
        if (result === 'Avatar uploaded successfully') {
          this._dialogRef.close(this.imgfile);
        } else {
          this.errorMsg.set(result || 'Upload failed');
        }
      });
    }
  }

  onNoClick(): void {
    this._dialogRef.close();
  }

  onFileSelected(event: Event): void {
    const input = event.target as HTMLInputElement;
    if (input.files && input.files[0]) {
      const file = input.files[0];
      if (this.acceptedMimetype.includes(file.type)) {
        this.imgfile = file;
        this.errorMsg.set(undefined);
       
        const reader = new FileReader();
        reader.onload = () => this.imgPreview.set(reader.result as string);
        reader.readAsDataURL(file);
      } else {
        this.errorMsg.set('Invalid file type. Please select a PNG, JPG, or JPEG image.');
      }
    }
  }
} 
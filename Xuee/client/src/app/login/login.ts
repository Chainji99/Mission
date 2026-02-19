
import { Component, signal, inject } from '@angular/core';
import {
  FormBuilder,
  FormGroup,
  FormControl,
  Validators,
  ValidationErrors,
  ValidatorFn,
  AbstractControl,
  ReactiveFormsModule
} from '@angular/forms';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { Router } from '@angular/router';
import { PassportService } from '../_services/passport-service';


export const PasswordValidator = (minLength: number, maxLength: number): ValidatorFn => {
  return (control: AbstractControl): ValidationErrors | null => {
    const password = control.value as string;
    if (!password) return { required: true };
    else if (password.length < minLength) return { invalidMinlength: true };
    else if (password.length > maxLength) return { invalidMaxlength: true };
    else if (!/[a-z]/.test(password)) return { invalidLowerCase: true };
    else if (!/[A-Z]/.test(password)) return { invalidUpperCase: true };
    else if (!/[0-9]/.test(password)) return { invalidNumeric: true };
    else if (!/[@#$%^&*(),.?":{}|<>]/.test(password)) return { invalidSpecialChar: true };
    return null;
  };
};

export function PasswordMatchValidator(
  ctrl_password_name: string,
  ctrl_confirm_password_name: string
): ValidatorFn {
  return (formGroup: AbstractControl): ValidationErrors | null => {
    const ctrlPassword = formGroup.get(ctrl_password_name);
    const ctrlConfirmPassword = formGroup.get(ctrl_confirm_password_name);

    if (!ctrlPassword || !ctrlConfirmPassword) return null;

    if (ctrlPassword.value !== ctrlConfirmPassword.value) {
      ctrlConfirmPassword.setErrors({ mismatch: true });
    } else {
      ctrlConfirmPassword.setErrors(null);
    }

    return null;
  };
}

@Component({
  selector: 'app-login',
  templateUrl: './login.html',
  styleUrls: ['./login.scss'],
  imports: [ReactiveFormsModule, MatFormFieldModule, MatInputModule, MatButtonModule, MatCardModule]
})
export class Login {
  private _passport = inject(PassportService);
  private _router = inject(Router);

  mode: 'login' | 'register' | 'forgot-password' = 'login';
  isResetSent = signal(false);

  form: FormGroup = new FormGroup({});

  // ข้อความแสดงข้อผิดพลาด (ใช้ Signals สำหรับ reactive UI)
  errorMessage = {
    username: signal(''),
    password: signal(''),
    confirm_password: signal(''),
    display_name: signal('')
  };

  errorFromServer = '';
  isLoading = signal(false);

  constructor(private fb: FormBuilder) {
    this.initializeForm();
  }

  private initializeForm(): void {
    this.form = this.fb.group({
      username: ['', [
        Validators.required,
        Validators.minLength(4),
        Validators.maxLength(16)
      ]],
      password: ['', [
        Validators.required,
        PasswordValidator(8, 16)
      ]]
    });
  }

  toggleMode(newMode: 'login' | 'register' | 'forgot-password'): void {
    this.mode = newMode;
    this.isResetSent.set(false);
    this.errorFromServer = '';
    this.updateForm();
  }

  private updateForm(): void {
    if (this.mode === 'register') {
      // เพิ่มฟิลด์สำหรับสมัครสมาชิก
      if (!this.form.contains('confirm_password')) {
        this.form.addControl('confirm_password', new FormControl('', Validators.required));
      }
      if (!this.form.contains('display_name')) {
        this.form.addControl('display_name', new FormControl('', Validators.required));
      }
      if (!this.form.contains('password')) {
        this.form.addControl('password', new FormControl('', [Validators.required, PasswordValidator(8, 16)]));
      }

      // เพิ่ม validator ตรวจสอบรหัสผ่านตรงกัน
      this.form.addValidators(PasswordMatchValidator('password', 'confirm_password'));
    } else if (this.mode === 'forgot-password') {
      // เฉพาะ username
      this.form.removeControl('confirm_password');
      this.form.removeControl('display_name');
      this.form.removeControl('password');
      this.form.removeValidators(PasswordMatchValidator('password', 'confirm_password'));
    } else {
      // ล็อกอิน
      if (!this.form.contains('password')) {
        this.form.addControl('password', new FormControl('', [Validators.required, PasswordValidator(8, 16)]));
      }
      this.form.removeControl('confirm_password');
      this.form.removeControl('display_name');

      // ลบ validator ที่เกี่ยวข้อง
      this.form.removeValidators(PasswordMatchValidator('password', 'confirm_password'));
    }

    // รีเซ็ต error messages
    Object.values(this.errorMessage).forEach(err => err.set(''));
  }

  updateErrorMessage(ctrlName: string): void {
    const control = this.form.get(ctrlName);
    if (!control) return;

    switch (ctrlName) {
      case 'username':
        if (control.hasError('required')) {
          this.errorMessage.username.set('Username is required');
        } else if (control.hasError('minlength')) {
          this.errorMessage.username.set('At least 4 characters');
        } else if (control.hasError('maxlength')) {
          this.errorMessage.username.set('Maximum 16 characters');
        } else {
          this.errorMessage.username.set('');
        }
        break;

      case 'password':
        if (control.hasError('required')) {
          this.errorMessage.password.set('Password is required');
        } else if (control.hasError('invalidMinlength')) {
          this.errorMessage.password.set('At least 8 characters');
        } else if (control.hasError('invalidMaxlength')) {
          this.errorMessage.password.set('Maximum 16 characters');
        } else if (control.hasError('invalidLowerCase')) {
          this.errorMessage.password.set('Add lowercase letter (a-z)');
        } else if (control.hasError('invalidUpperCase')) {
          this.errorMessage.password.set('Add uppercase letter (A-Z)');
        } else if (control.hasError('invalidNumeric')) {
          this.errorMessage.password.set('Add a number (0-9)');
        } else if (control.hasError('invalidSpecialChar')) {
          this.errorMessage.password.set('Add special character (!@#$%^&*)');
        } else {
          this.errorMessage.password.set('');
        }
        break;

      case 'confirm_password':
        if (control.hasError('required')) {
          this.errorMessage.confirm_password.set('Confirm password is required');
        } else if (control.hasError('mismatch')) {
          this.errorMessage.confirm_password.set('Passwords do not match');
        } else {
          this.errorMessage.confirm_password.set('');
        }
        break;

      case 'display_name':
        if (control.hasError('required')) {
          this.errorMessage.display_name.set('Display name is required');
        } else {
          this.errorMessage.display_name.set('');
        }
        break;
    }
  }

  async onSubmit() {
    if (this.form.invalid) return;

    this.isLoading.set(true);
    const formData = this.form.value;
    let error = '';

    try {
      if (this.mode === 'login') {
        error = await this._passport.get({
          username: formData.username,
          password: formData.password
        });
      } else if (this.mode === 'register') {
        error = await this._passport.new({
          username: formData.username,
          password: formData.password,
          display_name: formData.display_name
        });
      } else if (this.mode === 'forgot-password') {
        error = await this._passport.requestPasswordReset(formData.username);
        if (!error) {
          this.isResetSent.set(true);
        }
      }

      if (error) {
        this.errorFromServer = error;
      } else {
        this._router.navigate(['/']);
      }
    } finally {
      this.isLoading.set(false);
    }
  }

  async onGoogleLogin() {
    this.isLoading.set(true);
    try {
      const url = await this._passport.getGoogleAuthUrl();
      window.location.href = url;
    } catch (error) {
      console.error('Failed to get Google Auth URL', error);
      this.errorFromServer = this._passport.formatError(error);
    } finally {
      this.isLoading.set(false);
    }
  }
}
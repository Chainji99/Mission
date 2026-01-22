
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
  imports: [ReactiveFormsModule, MatFormFieldModule, MatInputModule, MatButtonModule]
})
export class Login {
  mode: 'login' | 'register' = 'login'; // เริ่มต้นเป็นโหมดล็อกอิน

  form: FormGroup = new FormGroup({});

  // ข้อความแสดงข้อผิดพลาด (ใช้ Signals สำหรับ reactive UI)
  errorMessage = {
    username: signal(''),
    password: signal(''),
    confirm_password: signal(''),
    display_name: signal('')
  };

  errorFromServer = '';

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

  toggleMode(): void {
    this.mode = this.mode === 'login' ? 'register' : 'login';
    this.updateForm();
  }

  private updateForm(): void {
    if (this.mode === 'register') {
      // เพิ่มฟิลด์สำหรับสมัครสมาชิก
      this.form.addControl('confirm_password', new FormControl('', Validators.required));
      this.form.addControl('display_name', new FormControl('', Validators.required));

      // เพิ่ม validator ตรวจสอบรหัสผ่านตรงกัน
      this.form.addValidators(PasswordMatchValidator('password', 'confirm_password'));
    } else {
      // ลบฟิลด์ที่ไม่จำเป็นเมื่อกลับไปโหมดล็อกอิน
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
          this.errorMessage.username.set('Must be at least 4 characters long');
        } else if (control.hasError('maxlength')) {
          this.errorMessage.username.set('Must be 16 characters or fewer');
        } else {
          this.errorMessage.username.set('');
        }
        break;

      case 'password':
        if (control.hasError('required')) {
          this.errorMessage.password.set('Password is required');
        } else if (control.hasError('invalidMinlength')) {
          this.errorMessage.password.set('Password must be at least 8 characters');
        } else if (control.hasError('invalidMaxlength')) {
          this.errorMessage.password.set('Password must be 16 characters or fewer');
        } else if (control.hasError('invalidLowerCase')) {
          this.errorMessage.password.set('Password must contain at least one lowercase letter');
        } else if (control.hasError('invalidUpperCase')) {
          this.errorMessage.password.set('Password must contain at least one uppercase letter');
        } else if (control.hasError('invalidNumeric')) {
          this.errorMessage.password.set('Password must contain at least one number');
        } else if (control.hasError('invalidSpecialChar')) {
          this.errorMessage.password.set('Password must contain at least one special character (@#$%^&* etc.)');
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

  updateErrorMessages(control: AbstractControl, fieldName: string): void {
    switch (fieldName) {
      case 'username':
        if (control.hasError('required')) {
          this.errorMessage.username.set('required');
        } else if (control.hasError('minlength')) {
          this.errorMessage.username.set('must be at least 4 characters long');
        } else if (control.hasError('maxlength')) { 
          this.errorMessage.username.set('must be 16 characters or fewer');
        } else {
          this.errorMessage.username.set('');
        }
        break;

      case 'password':
        if (control.hasError('required')) {
          this.errorMessage.password.set('required');
        } else if (control.hasError('invalidMinlength')) {
          this.errorMessage.password.set('must be at least 8 characters long');
        } else if (control.hasError('invalidMaxlength')) {
          this.errorMessage.password.set('must be 16 characters or fewer');
        } else if (control.hasError('invalidLowerCase')) {
          this.errorMessage.password.set('must contain minimum of 1 lower-case letter [a-z].');
        } else if (control.hasError('invalidUpperCase')) {
          this.errorMessage.password.set('must contain minimum of 1 capital letter [A-Z].');
        } else if (control.hasError('invalidNumeric')) {
          this.errorMessage.password.set('must contain minimum of 1 numeric character [0-9].');
        } else if (control.hasError('invalidSpecialChar')) {
          this.errorMessage.password.set('must contain minimum of 1 special character: !@#$%^&*(),.?":{}|<>');
        } else {
          this.errorMessage.password.set('');
        }
        break;

      case 'confirm_password':
        if (control.hasError('required')) {
          this.errorMessage.confirm_password.set('required'); // ⚠️ แก้จาก .password เป็น .confirm_password
        } else if (control.hasError('mismatch')) {
          this.errorMessage.confirm_password.set('do not match password');
        } else {
          this.errorMessage.confirm_password.set('');
        }
        break;
    }
  }

  onSubmit(): void {
    if (this.form.valid) {
      const formData = this.form.value;
      if (this.mode === 'login') {
        console.log('Login with:', formData);
        // TODO: เรียก AuthService.login(formData.username, formData.password)
      } else {
        console.log('Register with:', formData);
        // TODO: เรียก AuthService.register(formData)
      }
    }
  }
}
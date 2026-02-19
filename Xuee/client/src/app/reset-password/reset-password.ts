
import { Component, signal, inject, OnInit } from '@angular/core';
import {
    FormBuilder,
    FormGroup,
    Validators,
    ReactiveFormsModule
} from '@angular/forms';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { ActivatedRoute, Router } from '@angular/router';
import { PassportService } from '../_services/passport-service';
import { PasswordValidator, PasswordMatchValidator } from '../login/login';

@Component({
    selector: 'app-reset-password',
    templateUrl: './reset-password.html',
    styleUrls: ['./reset-password.scss'],
    imports: [ReactiveFormsModule, MatFormFieldModule, MatInputModule, MatButtonModule, MatCardModule]
})
export class ResetPassword implements OnInit {
    private _passport = inject(PassportService);
    private _router = inject(Router);
    private _route = inject(ActivatedRoute);

    form: FormGroup = new FormGroup({});
    token: string | null = null;

    isResetSuccess = signal(false);
    isLoading = signal(false);
    errorFromServer = '';

    errorMessage = {
        password: signal(''),
        confirm_password: signal('')
    };

    constructor(private fb: FormBuilder) {
        this.initializeForm();
    }

    ngOnInit(): void {
        this.token = this._route.snapshot.queryParamMap.get('token');
        if (!this.token) {
            this.errorFromServer = 'Invalid or missing reset token.';
        }
    }

    private initializeForm(): void {
        this.form = this.fb.group({
            password: ['', [
                Validators.required,
                PasswordValidator(8, 16)
            ]],
            confirm_password: ['', Validators.required]
        }, {
            validators: PasswordMatchValidator('password', 'confirm_password')
        });
    }

    updateErrorMessage(ctrlName: string): void {
        const control = this.form.get(ctrlName);
        if (!control) return;

        if (ctrlName === 'password') {
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
        } else if (ctrlName === 'confirm_password') {
            if (control.hasError('required')) {
                this.errorMessage.confirm_password.set('Confirm password is required');
            } else if (control.hasError('mismatch')) {
                this.errorMessage.confirm_password.set('Passwords do not match');
            } else {
                this.errorMessage.confirm_password.set('');
            }
        }
    }

    async onSubmit() {
        if (this.form.invalid || !this.token) return;

        this.isLoading.set(true);
        const password = this.form.value.password;

        try {
            const error = await this._passport.resetPassword(this.token, password);
            if (error) {
                this.errorFromServer = error;
            } else {
                this.isResetSuccess.set(true);
            }
        } finally {
            this.isLoading.set(false);
        }
    }

    goToLogin() {
        this._router.navigate(['/login']);
    }
}

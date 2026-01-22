import { inject, Injectable } from '@angular/core';
import { MatSnackBar, MatSnackBarConfig } from '@angular/material/snack-bar';
import { NavigationExtras, Router } from '@angular/router';
import { Observable, throwError } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class ErrorService {
  private _router = inject(Router);
  private _snackBar = inject(MatSnackBar);

  private _snackBarConfig: MatSnackBarConfig = {
    horizontalPosition: 'right',
    verticalPosition: 'top'
  };

  handleError(error: any): Observable<never> {
    if (error) {
        switch (error.status) {
            case 400:
                if (error.error.errors) 
                    this._snackBar.open(error.error.message, 'ok' , this._snackBarConfig);
                else 
                    this._snackBar.open(error.error.message || 'Bad Request.', 'ok', this._snackBarConfig);
                break
            case 404:
                break
            case 500:
            case 501:
            case 502:
            case 503:
            case 504:
            case 505:
            case 506:
            case 507:
            case 508:
            case 509:
            case 510:
            case 511:
                const navExtras: NavigationExtras = {
                    state: { error: error.error }
                }
                this._router.navigateByUrl('/server-error', navExtras);
                this._snackBar.open('Server Error.', 'Close', this._snackBarConfig);
                break
                default:
                this._snackBar.open('An unexpected error occurred.', 'Close', this._snackBarConfig);
                break
        }
    }

    return throwError(() => error);
  }

}
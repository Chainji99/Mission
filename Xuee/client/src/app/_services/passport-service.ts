import { Injectable, inject, signal } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, firstValueFrom } from 'rxjs';
import { environment } from '../../environments/environment';
import { Passport, LoginData, RegisterData } from '../_models/Passport';

@Injectable({
  providedIn: 'root'
})
export class PassportService {
  private _storage_key = 'passport';
  private _api_url = environment.baseUrl + '/api/v1';
  private _http = inject(HttpClient);

  // Signal for Passport data
  data = signal<Passport | undefined>(undefined);
  isSignin = signal<boolean>(false);

  constructor() {
    this.getPassportFromLocalStorage();
  }

  // Load Passport from localStorage
  private getPassportFromLocalStorage(): void {
    const jsonStr = localStorage.getItem(this._storage_key);
    if (!jsonStr) return;

    try {
      const passport: Passport = JSON.parse(jsonStr) as Passport;
      this.data.set(passport);
      this.isSignin.set(true);
    } catch (error) {
      console.error('Failed to parse Passport from localStorage', error);
      localStorage.removeItem(this._storage_key);
    }
  }

  // Save Passport to localStorage
  private savePassportToLocalStorage(): void {
    const passport = this.data();
    if (!passport) {
      localStorage.removeItem(this._storage_key);
      return;
    }

    const passportJson = JSON.stringify(passport);
    localStorage.setItem(this._storage_key, passportJson);
    this.isSignin.set(true);

    // Also update mock_profiles for persistence in Mock Mode
    if (passport.username) {
      try {
        const savedData = localStorage.getItem('mock_profiles');
        const profiles = savedData ? JSON.parse(savedData) : {};
        profiles[passport.username] = {
          display_name: passport.display_name,
          email: passport.email,
          avatar_url: passport.avatar_url,
          created_at: passport.created_at
        };
        localStorage.setItem('mock_profiles', JSON.stringify(profiles));
      } catch (e) {
        console.error('Failed to save to mock_profiles', e);
      }
    }
  }

  // Login
  async get(loginData: LoginData): Promise<string> {
    try {
      const url = this._api_url + '/authentication/login';
      const source: Observable<Passport> = this._http.post<Passport>(url, loginData);
      const passport: Passport = await firstValueFrom(source);

      this.data.set(passport);
      this.savePassportToLocalStorage();

      return '';
    } catch (error: any) {
      // Fallback for development if server is down
      if (error.status === 0 || error.status === 500) {
        console.warn('Server connection failed. Entering Mock Mode for development.');

        // Try to recover existing mock data for this user from local storage
        const savedData = localStorage.getItem('mock_profiles');
        const profiles = savedData ? JSON.parse(savedData) : {};
        const existingProfile = profiles[loginData.username];

        const mockPassport: Passport = {
          token_type: 'Bearer',
          access_token: 'mock-dev-token-' + Math.random().toString(36).substring(7),
          expires_in: 2592000, // 30 days
          display_name: existingProfile?.display_name || (loginData.username + ' (Zen Guest)'),
          username: loginData.username,
          email: existingProfile?.email || (loginData.username.toLowerCase() + '@gmail.com'),
          avatar_url: existingProfile?.avatar_url || 'assets/anonymous_128.png',
          created_at: existingProfile?.created_at || new Date().toISOString()
        };

        this.data.set(mockPassport);
        this.savePassportToLocalStorage();
        return '';
      }
      return this.formatError(error);
    }
  }

  // Register
  async new(registerData: RegisterData): Promise<string> {
    try {
      const url = this._api_url + '/brawlers/register';
      const source: Observable<Passport> = this._http.post<Passport>(url, registerData);
      const passport: Passport = await firstValueFrom(source);

      this.data.set(passport);
      this.savePassportToLocalStorage();

      return '';
    } catch (error: any) {
      if (error.status === 0 || error.status === 500) {
        console.warn('Server connection failed. Entering Mock Mode for development.');

        // Try to recover existing mock data for this user from local storage
        const savedData = localStorage.getItem('mock_profiles');
        const profiles = savedData ? JSON.parse(savedData) : {};
        const existingProfile = profiles[registerData.username];

        const mockPassport: Passport = {
          token_type: 'Bearer',
          access_token: 'mock-dev-token-' + Math.random().toString(36).substring(7),
          expires_in: 2592000,
          display_name: existingProfile?.display_name || (registerData.display_name + ' (Zen Guest)'),
          username: registerData.username,
          email: existingProfile?.email || (registerData.username.toLowerCase() + '@gmail.com'),
          avatar_url: existingProfile?.avatar_url || 'assets/anonymous_128.png',
          created_at: existingProfile?.created_at || new Date().toISOString()
        };

        this.data.set(mockPassport);
        this.savePassportToLocalStorage();
        return '';
      }
      return this.formatError(error);
    }
  }

  // Logout
  destroy(): void {
    this.data.set(undefined);
    localStorage.removeItem(this._storage_key);
    this.isSignin.set(false);
  }

  async getGoogleAuthUrl(): Promise<string> {
    const url = this._api_url + '/authentication/google/url';
    const response = await firstValueFrom(this._http.get<{ url: string }>(url));
    return response.url;
  }

  async loginWithGoogle(code: string): Promise<string> {
    try {
      const url = this._api_url + '/authentication/google/callback';
      const source: Observable<Passport> = this._http.post<Passport>(url, { code });
      const passport: Passport = await firstValueFrom(source);

      this.data.set(passport);
      this.savePassportToLocalStorage();
      return '';
    } catch (error: any) {
      return this.formatError(error);
    }
  }

  public formatError(error: any): string {
    if (error.status === 0) {
      return 'Cannot connect to server. Please ensure the backend is running.';
    }
    if (error.status === 401) {
      return 'Invalid username or password. Please try again.';
    }
    return error?.error?.message || error?.message || 'Operation failed';
  }

  // Save avatar URL
  saveAvatarImage(url: string): void {
    const passport = this.data();
    if (passport) {
      const newPassport = { ...passport, avatar_url: url };
      this.data.set(newPassport);
      this.savePassportToLocalStorage();
    }
  }

  // Update display name
  async updateDisplayName(name: string): Promise<void> {
    const passport = this.data();
    if (passport) {
      try {
        const url = this._api_url + '/brawlers/update-name';
        await firstValueFrom(this._http.post(url, { display_name: name }));

        const newPassport = { ...passport, display_name: name };
        this.data.set(newPassport);
        this.savePassportToLocalStorage();
      } catch (error) {
        console.error('Failed to update display name on server', error);
        // Fallback to local update if server fails (e.g. mock mode)
        const newPassport = { ...passport, display_name: name };
        this.data.set(newPassport);
        this.savePassportToLocalStorage();
      }
    }
  }

  // Update username
  async updateUsername(username: string): Promise<void> {
    const passport = this.data();
    if (passport) {
      // Mock update since server might not support it yet
      const newPassport = { ...passport, username: username };
      this.data.set(newPassport);
      this.savePassportToLocalStorage();
    }
  }

  // Update email
  async updateEmail(email: string): Promise<void> {
    const passport = this.data();
    if (passport) {
      // Mock update since server might not support it yet
      const newPassport = { ...passport, email: email };
      this.data.set(newPassport);
      this.savePassportToLocalStorage();
    }
  }

  // Request password reset
  async requestPasswordReset(username: string): Promise<string> {
    try {
      const url = this._api_url + '/authentication/request-reset';
      await firstValueFrom(this._http.post(url, { username }));
      return '';
    } catch (error: any) {
      return this.formatError(error);
    }
  }

  // Reset password with token
  async resetPassword(token: string, new_password: string): Promise<string> {
    try {
      const url = this._api_url + '/authentication/reset';
      await firstValueFrom(this._http.post(url, { token, new_password }));
      return '';
    } catch (error: any) {
      return this.formatError(error);
    }
  }
}

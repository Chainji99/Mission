import { Injectable, inject, signal } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, firstValueFrom } from 'rxjs';
import { environment } from '../../environments/environment';


export interface Passport {
  token_type: string,
  access_token: string,
  expires_in: number,
  display_name: string,
  avatar_url?: string, 
}


export interface LoginData {
  username: string,
  password: string
}


export interface RegisterData {
  username: string,
  password: string,
  display_name: string
}

@Injectable({
  providedIn: 'root'
})
export class PassportService {
  private _storage_key = 'passport';
  private _api_url = environment.baseUrl + '/api/v1';
  private _http = inject(HttpClient);

  // Signal สำหรับเก็บข้อมูล Passport (ใช้ใน Angular 16+)
  data = signal<Passport | undefined>(undefined);

  constructor() {
    this.getPassportFromLocalStorage();
  }

  // 🔹 ฟังก์ชัน: โหลดข้อมูล Passport จาก localStorage
  private getPassportFromLocalStorage(): void {
    const jsonStr = localStorage.getItem(this._storage_key);
    if (!jsonStr) return;

    try {
      const passport: Passport = JSON.parse(jsonStr) as Passport;
      this.data.set(passport);
    } catch (error) {
      console.error('Failed to parse Passport from localStorage', error);
    }
  }

  // 🔹 ฟังก์ชัน: บันทึกข้อมูล Passport ลง localStorage
  private savePassportToLocalStorage(): void {
    const passport = this.data();
    if (!passport) return;

    const passportJson = JSON.stringify(passport);
    localStorage.setItem(this._storage_key, passportJson);
  }

  // 🔹 ฟังก์ชัน: ล็อกอินผ่าน API
  async get(loginData: LoginData): Promise<string> {
    try {
      const url = this._api_url + '/authentication/login';
      const source: Observable<Passport> = this._http.post<Passport>(url, loginData);
      const passport: Passport = await firstValueFrom(source);

      // อัปเดตข้อมูลใน service และบันทึกลง localStorage
      this.data.set(passport);
      this.savePassportToLocalStorage();

      return ''; // คืนค่า string ว่างเมื่อสำเร็จ (ตามที่กำหนดใน signature)
    } catch (error: any) {
      // คืนค่าข้อความ error จาก server (เช่น error.error.message)
      return error?.error || 'Login failed';
    }
  }

  // 🔹 ฟังก์ชัน: ล็อกเอาท์
  logout(): void {
    this.data.set(undefined);
    localStorage.removeItem(this._storage_key);
  }
}
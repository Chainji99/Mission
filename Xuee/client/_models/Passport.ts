import { } from '@angular/forms';
export interface Passport {
  token_type: string,
  access_token: string,
  expires_in: number,
  display_name: string,
  avatar_url?: string, // ค่าไม่จำเป็นต้องมี (optional)
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
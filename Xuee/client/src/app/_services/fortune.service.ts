import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { environment } from '../../environments/environment';
import { firstValueFrom } from 'rxjs';

export interface FortuneStick {
  id: number;
  number: number;
  poem_text: string;
  interpretation: string;
  lucky_direction?: string;
}

export interface DailyFortune {
  id: number;
  user_id: number;
  stick_id: number;
  created_at: string;
  date: string;
}

export interface DailyFortuneDetail {
  daily_fortune: DailyFortune;
  stick: FortuneStick;
}

@Injectable({
  providedIn: 'root'
})
export class FortuneService {
  private http = inject(HttpClient);
  private apiUrl = `${environment.api}/fortune`;

  // Get daily fortune (draw new or get existing)
  async getDailyFortune(): Promise<DailyFortuneDetail> {
    return await firstValueFrom(this.http.get<DailyFortuneDetail>(`${this.apiUrl}/daily`));
  }

  // Draw a random fortune without saving (for anonymous/public use)
  async drawRandomFortune(): Promise<FortuneStick> {
    return await firstValueFrom(this.http.get<FortuneStick>(`${this.apiUrl}/draw`));
  }
}

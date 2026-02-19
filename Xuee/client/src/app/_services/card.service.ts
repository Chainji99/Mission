import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { environment } from '../../environments/environment';
import { firstValueFrom } from 'rxjs';

export interface Card {
  id: number;
  name: string;
  language: string;
  rarity: string;
  attack: number;
  defense: number;
  image_url?: string;
}

export interface UserCard {
  id: number;
  user_id: number;
  card_id: number;
  level: number;
  experience: number;
  obtained_at: string;
}

export interface UserCardDetail {
  user_card: UserCard;
  card: Card;
}

@Injectable({
  providedIn: 'root'
})
export class CardService {
  private http = inject(HttpClient);
  private apiUrl = `${environment.api}/cards`;

  // Public: Get all cards (Gallery)
  async getAllCards(): Promise<Card[]> {
    return await firstValueFrom(this.http.get<Card[]>(this.apiUrl));
  }

  // Private: Get user's inventory
  async getMyInventory(): Promise<UserCardDetail[]> {
    return await firstValueFrom(this.http.get<UserCardDetail[]>(`${this.apiUrl}/inventory`));
  }

  // Private: Draw Gacha
  async drawGacha(): Promise<UserCardDetail> {
    return await firstValueFrom(this.http.post<UserCardDetail>(`${this.apiUrl}/gacha`, {}));
  }

  // Private: Upgrade Card
  async upgradeCard(userCardId: number, amount: number): Promise<{ message: string }> {
    return await firstValueFrom(this.http.post<{ message: string }>(`${this.apiUrl}/upgrade`, { user_card_id: userCardId, amount }));
  }

  getLanguageLogo(lang: string): string {
    const l = lang.toLowerCase().trim();
    const mapping: { [key: string]: string } = {
      'python': 'python/python-original.svg',
      'javascript': 'javascript/javascript-original.svg',
      'typescript': 'typescript/typescript-original.svg',
      'rust': 'rust/rust-original.svg',
      'go': 'go/go-original-wordmark.svg',
      'java': 'java/java-original.svg',
      'c++': 'cplusplus/cplusplus-original.svg',
      'c#': 'csharp/csharp-original.svg',
      'php': 'php/php-original.svg',
      'swift': 'swift/swift-original.svg',
      'kotlin': 'kotlin/kotlin-original.svg',
      'ruby': 'ruby/ruby-original.svg',
      'dart': 'dart/dart-original.svg',
      'html/css': 'html5/html5-original.svg',
      'sql': 'postgresql/postgresql-original.svg',
      'shell': 'bash/bash-original.svg'
    };

    const slug = mapping[l] || 'javascript/javascript-original.svg';
    return `https://cdn.jsdelivr.net/gh/devicons/devicon/icons/${slug}`;
  }
}

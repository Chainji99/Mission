import { Component, signal, inject, OnInit, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { RouterModule, Router } from '@angular/router';
import { CardService, UserCardDetail, Card } from '../_services/card.service';
import { PassportService } from '../_services/passport-service';

@Component({
  selector: 'app-gacha',
  templateUrl: './gacha.html',
  styleUrls: ['./gacha.scss'],
  standalone: true,
  imports: [CommonModule, MatButtonModule, RouterModule]
})
export class GachaComponent implements OnInit {
  private cardService = inject(CardService);
  private passportService = inject(PassportService);
  private router = inject(Router);
  
  isLoading = signal(false);
  result = signal<UserCardDetail | null>(null);
  
  // Wheel Rotation Logic
  private rotation = signal(0);
  wheelRotation = computed(() => `rotate(${this.rotation()}deg)`);

  // Featured cards for display
  featuredCards = signal<Card[]>([]);
  isLoggedIn = computed(() => this.passportService.isSignin());

  // Gacha Rates for Display
  rarityRates = [
    { grade: 'Legendary', rate: '2%', color: '#b22222', description: 'สุดยอดการ์ดในตำนาน' },
    { grade: 'Epic', rate: '8%', color: '#1a1a1a', description: 'การ์ดระดับสูงที่หายาก' },
    { grade: 'Rare', rate: '20%', color: '#555555', description: 'การ์ดระดับกลาง' },
    { grade: 'Common', rate: '70%', color: '#9e9e9e', description: 'การ์ดทั่วไป' }
  ];

  async ngOnInit() {
    this.loadFeaturedCards();
  }

  async loadFeaturedCards() {
    try {
      const allCards = await this.cardService.getAllCards();
      // Randomly pick 3 cards to showcase
      const shuffled = [...allCards].sort(() => 0.5 - Math.random());
      this.featuredCards.set(shuffled.slice(0, 3));
    } catch (error) {
      console.warn('Failed to load real featured cards, using mocks', error);
      // Fallback: Generate some mock cards for display
      const mocks = Array.from({ length: 3 }, () => this.generateMockCard());
      this.featuredCards.set(mocks);
    }
  }

  async onDraw() {
    if (this.isLoading()) return;
    
    this.isLoading.set(true);
    let cardToReveal: UserCardDetail | null = null;

    try {
      // 1. Get the result first (so we know where to stop the wheel)
      if (this.isLoggedIn()) {
        try {
          cardToReveal = await this.cardService.drawGacha();
        } catch (serverError) {
          console.warn('Server gacha failed, falling back to mock draw', serverError);
          cardToReveal = this.generateMockUserCardDetail();
        }
      } else {
        cardToReveal = this.generateMockUserCardDetail();
      }

      // 2. Calculate target rotation
      // Add 5-10 full spins for effect + target segment
      const fullSpins = 5 + Math.floor(Math.random() * 5);
      const rarity = cardToReveal.card.rarity.toLowerCase();
      
      let baseAngle = 0;
      if (rarity === 'legendary') baseAngle = 315;
      else if (rarity === 'epic') baseAngle = 225;
      else if (rarity === 'rare') baseAngle = 135;
      else baseAngle = 45;

      // Add slight randomness within the 90deg segment (±30deg from center)
      const offset = (Math.random() - 0.5) * 60;
      const targetRotation = (fullSpins * 360) + baseAngle + offset;
      
      // Update rotation
      this.rotation.set(targetRotation);

      // 3. Wait for animation (4s in SCSS)
      await new Promise(resolve => setTimeout(resolve, 4100));
      
      // 4. Show result
      this.result.set(cardToReveal);
      
    } catch (error) {
      console.error('Gacha system error', error);
      alert('Failed to draw card. Please try again.');
    } finally {
      this.isLoading.set(false);
    }
  }

  private generateMockUserCardDetail(): UserCardDetail {
    const mockCard = this.generateMockCard();
    return {
      card: mockCard,
      user_card: {
        id: 0,
        user_id: 0,
        card_id: mockCard.id,
        level: 1,
        experience: 0,
        obtained_at: new Date().toISOString()
      }
    };
  }

  // Temporary mock generator for public users
  private generateMockCard(): Card {
    const languages = [
      'Python', 'JavaScript', 'Rust', 'Go', 'Java', 'TypeScript', 'C++', 'C#', 
      'PHP', 'Swift', 'Kotlin', 'Ruby', 'Dart', 'Scala', 'Haskell', 'Lua',
      'Assembly', 'SQL', 'HTML/CSS', 'Shell', 'R', 'Julia', 'Zig', 'Elixir'
    ];
    const randomLang = languages[Math.floor(Math.random() * languages.length)];
    
    // Use weighted random for mock draw too
    const chance = Math.random() * 100;
    let randomRarity = 'Common';
    if (chance < 2) randomRarity = 'Legendary';
    else if (chance < 10) randomRarity = 'Epic';
    else if (chance < 30) randomRarity = 'Rare';
    
    return {
      id: 0,
      name: `${randomLang}`,
      language: randomLang,
      rarity: randomRarity,
      attack: Math.floor(Math.random() * 100),
      defense: Math.floor(Math.random() * 100),
      image_url: ''
    };
  }

  reset() {
    this.result.set(null);
    this.rotation.set(0);
  }

  getLanguageLogo(lang: string): string {
    return this.cardService.getLanguageLogo(lang);
  }
}

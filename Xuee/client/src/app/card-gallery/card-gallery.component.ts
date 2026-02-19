import { Component, signal, computed, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { CardService, Card } from '../_services/card.service';

@Component({
  selector: 'app-card-gallery',
  templateUrl: './card-gallery.html',
  styleUrls: ['./card-gallery.scss'],
  standalone: true,
  imports: [CommonModule]
})
export class CardGalleryComponent implements OnInit {
  protected cardService = inject(CardService);
  cards = signal<Card[]>([]);
  isLoading = signal(true);
  
  // Search and Filter signals
  searchQuery = signal('');
  filterRarity = signal('All');

  filteredCards = computed(() => {
    let result = this.cards();
    
    // Filter by rarity
    if (this.filterRarity() !== 'All') {
      result = result.filter(c => c.rarity === this.filterRarity());
    }
    
    // Filter by search query
    const query = this.searchQuery().toLowerCase().trim();
    if (query) {
      result = result.filter(c => 
        c.name.toLowerCase().includes(query) || 
        c.language.toLowerCase().includes(query)
      );
    }
    
    return result;
  });

  async ngOnInit() {
    try {
      this.isLoading.set(true);
      const data = await this.cardService.getAllCards();
      this.cards.set(data);
    } catch (error) {
      console.error('Failed to load cards', error);
    } finally {
      this.isLoading.set(false);
    }
  }

  getLogo(lang: string) {
    return this.cardService.getLanguageLogo(lang);
  }

  onSearch(event: Event) {
    const input = event.target as HTMLInputElement;
    this.searchQuery.set(input.value);
  }

  setFilter(rarity: string) {
    this.filterRarity.set(rarity);
  }
}

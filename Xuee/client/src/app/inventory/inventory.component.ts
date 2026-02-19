import { Component, signal, inject, OnInit, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { RouterModule } from '@angular/router';
import { CardService, UserCardDetail } from '../_services/card.service';

@Component({
  selector: 'app-inventory',
  templateUrl: './inventory.html',
  styleUrls: ['./inventory.scss'],
  standalone: true,
  imports: [CommonModule, MatButtonModule, RouterModule]
})
export class InventoryComponent implements OnInit {
  private cardService = inject(CardService);
  inventory = signal<UserCardDetail[]>([]);

  // Unique inventory: show only the highest level card for each unique card ID
  uniqueInventory = computed(() => {
    const items = this.inventory();
    const uniqueMap = new Map<number, UserCardDetail>();

    items.forEach(item => {
      const existing = uniqueMap.get(item.card.id);
      if (!existing || item.user_card.level > existing.user_card.level) {
        uniqueMap.set(item.card.id, item);
      }
    });

    return Array.from(uniqueMap.values());
  });

  async ngOnInit() {
    this.loadInventory();
  }

  async loadInventory() {
    try {
      const data = await this.cardService.getMyInventory();
      this.inventory.set(data);
    } catch (error) {
      console.error('Failed to load inventory', error);
    }
  }

  async onUpgrade(item: UserCardDetail) {
    if (confirm(`Upgrade ${item.card.name}? (+10 EXP)`)) {
      try {
        await this.cardService.upgradeCard(item.user_card.id, 10);
        await this.loadInventory(); // Reload to show updates
      } catch (error) {
        alert('Upgrade failed');
      }
    }
  }

  getLanguageLogo(lang: string): string {
    return this.cardService.getLanguageLogo(lang);
  }
}

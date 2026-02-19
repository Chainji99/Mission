import { Injectable, inject, signal } from '@angular/core';
import { PassportService } from './passport-service';
import { FriendStore, FriendUser } from '../_models/friend';

@Injectable({ providedIn: 'root' })
export class FriendService {
  private _passport = inject(PassportService);
  private _storageKey = 'xue_friends_v2'; // New key to clear old baggage

  // Signals for state
  friends = signal<FriendUser[]>([]);
  outgoing = signal<FriendUser[]>([]);
  incoming = signal<FriendUser[]>([]);

  constructor() {
    this.load();

    // GUARANTEE mock data for testing if lists are empty
    if (this.friends().length === 0 && this.incoming().length === 0 && this.outgoing().length === 0) {
      console.log('FriendService: Initializing mock data...');
      const mockFriends: FriendUser[] = [
        { username: 'elon_musk', display_name: 'Elon Musk' },
        { username: 'mark_z', display_name: 'Mark Zuckerberg' },
        { username: 'steve_j', display_name: 'Steve Jobs' }
      ];
      const mockIncoming: FriendUser[] = [
        { username: 'dino_s', display_name: 'Dino Saur' },
        { username: 'rex_jr', display_name: 'Rex Junior' }
      ];
      this.friends.set(mockFriends);
      this.incoming.set(mockIncoming);
      this.save();
    }
  }

  private load() {
    try {
      const raw = localStorage.getItem(this._storageKey);
      if (raw) {
        const store = JSON.parse(raw) as FriendStore;
        this.friends.set(store.friends || []);
        this.outgoing.set(store.outgoing || []);
        this.incoming.set(store.incoming || []);
        console.log('FriendService: Loaded from storage');
      }
    } catch (e) {
      console.error('FriendService: Load error', e);
      this.clearLocal();
    }
  }

  private save() {
    const store: FriendStore = {
      friends: this.friends(),
      outgoing: this.outgoing(),
      incoming: this.incoming()
    };
    localStorage.setItem(this._storageKey, JSON.stringify(store));
  }

  private clearLocal() {
    this.friends.set([]);
    this.outgoing.set([]);
    this.incoming.set([]);
    localStorage.removeItem(this._storageKey);
  }

  async sendFriendRequest(username: string): Promise<string> {
    const cleaned = username.trim();
    if (!cleaned) return 'ระบุชื่อผู้ใช้';

    // Add to outgoing
    const newUser: FriendUser = { username: cleaned, display_name: cleaned };
    this.outgoing.update(list => [newUser, ...list]);
    this.save();
    return '';
  }

  accept(username: string) {
    const list = this.incoming();
    const u = list.find(x => x.username === username);
    if (!u) return;

    this.incoming.update(l => l.filter(x => x.username !== username));
    this.friends.update(l => [u, ...l]);
    this.save();
  }

  reject(username: string) {
    this.incoming.update(l => l.filter(x => x.username !== username));
    this.save();
  }

  cancel(username: string) {
    this.outgoing.update(l => l.filter(x => x.username !== username));
    this.save();
  }

  remove(username: string) {
    this.friends.update(l => l.filter(x => x.username !== username));
    this.save();
  }
}

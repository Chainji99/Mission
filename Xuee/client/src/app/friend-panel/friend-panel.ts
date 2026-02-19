import { Component, inject, ChangeDetectorRef } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { FriendService } from '../_services/friend-service';

@Component({
  selector: 'app-friend-panel',
  standalone: true,
  imports: [CommonModule, MatIconModule, MatButtonModule, MatInputModule, MatFormFieldModule, FormsModule],
  templateUrl: './friend-panel.html',
  styleUrls: ['./friend-panel.scss']
})
export class FriendPanel {
  private _friends = inject(FriendService);
  private _cdr = inject(ChangeDetectorRef);
  private _router = inject(Router);

  isExpanded = false;
  friendQuery = '';
  selectedTab: 'friends' | 'pending' | 'add' = 'friends';

  friendsList = this._friends.friends;
  outgoingList = this._friends.outgoing;
  incomingList = this._friends.incoming;

  toggleExpand() {
    this.isExpanded = !this.isExpanded;
  }

  async addFriendByUsername() {
    const query = this.friendQuery.trim();
    if (!query) return;
    const err = await this._friends.sendFriendRequest(query);
    if (err) alert(err);
    else alert('ส่งคำขอแล้ว');
    this.friendQuery = '';
  }

  startChat(username: string) {
    this._router.navigate([`/private-chat/${username}`]);
    this.isExpanded = false;
  }

  acceptFriend(username: string) { this._friends.accept(username); }
  rejectFriend(username: string) { this._friends.reject(username); }
  cancelOutgoing(username: string) { this._friends.cancel(username); }
  removeFriend(username: string) { this._friends.remove(username); }
}

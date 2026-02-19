import { Component, inject, ChangeDetectorRef, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { ChatService, ChatRoom } from '../_services/chat-service';
import { FriendService } from '../_services/friend-service';

@Component({
  selector: 'app-chat-panel',
  standalone: true,
  imports: [CommonModule, MatIconModule, MatButtonModule, MatInputModule, MatFormFieldModule, FormsModule],
  templateUrl: './chat-panel.html',
  styleUrls: ['./chat-panel.scss']
})
export class ChatPanel implements OnInit {
  _chat = inject(ChatService);
  private _friends = inject(FriendService);
  private _cdr = inject(ChangeDetectorRef);
  private _router = inject(Router);

  isExpanded = false;
  selectedTab: 'rooms' | 'friends' = 'rooms';

  chatRooms: ChatRoom[] = [];
  privateChats: any[] = [];
  friendsList = this._friends.friends;

  ngOnInit() {
    this._chat.rooms$.subscribe(rooms => { this.chatRooms = rooms; this._cdr.detectChanges(); });
    this._chat.privateChats$.subscribe(chats => { this.privateChats = chats; this._cdr.detectChanges(); });
  }

  toggleExpand() {
    this.isExpanded = !this.isExpanded;
  }

  selectTab(tab: 'rooms' | 'friends') {
    this.selectedTab = tab;
  }

  selectRoom(room: ChatRoom) {
    // Navigate to full-screen mission chat
    this._router.navigate([`/mission-chat/${room.id}`]);
    this.isExpanded = false;
  }

  startPrivateChat(friend: any) {
    // Navigate to full-screen private chat
    this._router.navigate([`/private-chat/${friend.username}`]);
    this.isExpanded = false;
  }

  closeChat(room: ChatRoom) {
    if (room.username) {
      this._chat.closePrivateChat(room.username);
    } else {
      this._chat.closeChat(room.id);
    }
  }
}

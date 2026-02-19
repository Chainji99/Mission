import { Component, inject, OnInit, OnDestroy, ViewChild, ElementRef } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatInputModule } from '@angular/material/input';
import { ChatService, ChatMessage } from '../_services/chat-service';
import { Subscription } from 'rxjs';

@Component({
  selector: 'app-mission-chat',
  standalone: true,
  imports: [CommonModule, FormsModule, MatButtonModule, MatIconModule, MatInputModule],
  templateUrl: './mission-chat.html',
  styleUrls: ['./mission-chat.scss']
})
export class MissionChat implements OnInit, OnDestroy {
  private _route = inject(ActivatedRoute);
  private _router = inject(Router);
  private _chatService = inject(ChatService);

  missionId!: number;
  title = 'Mission Chat';
  input = '';
  messages: ChatMessage[] = [];
  showMembers = false;

  // Mock 5 members in the group chat
  members = [
    { username: 'captain_nexus', display_name: 'Captain Nexus', role: 'Leader' },
    { username: 'iron_shield', display_name: 'Iron Shield', role: 'Tank' },
    { username: 'nova_strike', display_name: 'Nova Strike', role: 'DPS' },
    { username: 'healing_wind', display_name: 'Healing Wind', role: 'Support' },
    { username: 'shadow_step', display_name: 'Shadow Step', role: 'Infiltrator' }
  ];

  @ViewChild('messagesContainer') private _messagesContainer!: ElementRef<HTMLDivElement>;
  private _sub: Subscription | null = null;

  ngOnInit(): void {
    this._route.params.subscribe(params => {
      this.missionId = Number(params['id']);
      this.title = `Mission Chat #${this.missionId}`;
      this._subscribeToMessages();
    });
  }

  private _subscribeToMessages() {
    if (this._sub) this._sub.unsubscribe();
    this._sub = this._chatService.getMessages$(this.missionId).subscribe(msgs => {
      this.messages = msgs;
      setTimeout(() => this.scrollToBottom(), 50);
    });
  }

  toggleMembers() {
    this.showMembers = !this.showMembers;
  }

  send() {
    if (!this.input || this.input.trim() === '') return;
    this._chatService.sendMessage(this.missionId, this.input.trim());
    this.input = '';
  }

  close() {
    this._router.navigate(['/']); // Go back home or missions list
  }

  private scrollToBottom() {
    try {
      const el = this._messagesContainer?.nativeElement;
      if (el) el.scrollTop = el.scrollHeight;
    } catch (e) { }
  }

  ngOnDestroy(): void {
    if (this._sub) this._sub.unsubscribe();
  }
}

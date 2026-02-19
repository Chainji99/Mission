import { Injectable, inject } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { MissionService } from './mission-service';

export interface ChatRoom {
  id: number;
  name: string;
  username?: string; // For private chats
}

export interface ChatMessage {
  sender: string;
  text: string;
  ts: string;
}

@Injectable({ providedIn: 'root' })
export class ChatService {
  private _missionService = inject(MissionService);

  private _rooms = new BehaviorSubject<ChatRoom[]>([
    { id: 1, name: 'General Chat' },
    { id: 2, name: 'Global Mission' }
  ]);
  readonly rooms$ = this._rooms.asObservable();

  private _privateChats = new BehaviorSubject<Array<{ username: string; display_name: string }>>([]);
  readonly privateChats$ = this._privateChats.asObservable();

  private _currentRoom = new BehaviorSubject<ChatRoom | null>(null);
  readonly currentRoom$ = this._currentRoom.asObservable();

  private _messages = new BehaviorSubject<Array<{ id: number; text: string; timestamp: Date; isOwn: boolean }>>([]);
  readonly messages$ = this._messages.asObservable();

  private _roomMessages: Map<number | string, BehaviorSubject<ChatMessage[]>> = new Map();
  private _messageId = 0;

  constructor() {
    this._syncRoomsFromMyMissions();
  }

  private async _syncRoomsFromMyMissions() {
    try {
      const missions = await this._missionService.getMyMissions();
      if (missions && missions.length > 0) {
        const rooms = missions.map(m => ({ id: m.id, name: m.name }));
        this._rooms.next(rooms);
      }
    } catch {
      console.warn('ChatService: Mission sync failed, keeping defaults');
    }
  }

  sendMessage(roomId: number | string, text: string) {
    const msgId = ++this._messageId;
    const ts = new Date();

    // Update active stream
    const msg = { id: msgId, text, timestamp: ts, isOwn: true };
    this._messages.next([...this._messages.value, msg]);

    // Update persistent room stream
    const roomMsg: ChatMessage = { sender: 'You', text, ts: ts.toISOString() };
    if (!this._roomMessages.has(roomId)) {
      this._roomMessages.set(roomId, new BehaviorSubject<ChatMessage[]>([roomMsg]));
    } else {
      const subj = this._roomMessages.get(roomId)!;
      subj.next([...subj.value, roomMsg]);
    }
  }

  setCurrentRoom(room: ChatRoom) {
    this._currentRoom.next(room);
    const id = room.username || room.id;
    if (this._roomMessages.has(id)) {
      const msgs = this._roomMessages.get(id)!.value;
      this._messages.next(msgs.map((m, i) => ({
        id: i,
        text: m.text,
        timestamp: new Date(m.ts),
        isOwn: m.sender === 'You'
      })));
    } else {
      this._messages.next([]);
    }
  }

  openPrivateChat(user: { username: string; display_name: string }) {
    const list = this._privateChats.value;
    if (!list.find(u => u.username === user.username)) {
      this._privateChats.next([...list, user]);
    }
    this.setCurrentRoom({ id: -1, name: user.display_name, username: user.username });
  }

  openChat(mission: { id: number; name: string }) {
    const rooms = this._rooms.value;
    if (!rooms.find(r => r.id === mission.id)) {
      this._rooms.next([...rooms, mission]);
    }
    this.setCurrentRoom(mission);
  }

  setRoomsFromMissions(rooms: ChatRoom[]) {
    this._rooms.next(rooms);
  }

  getMessages$(roomId: number | string) {
    if (!this._roomMessages.has(roomId)) {
      this._roomMessages.set(roomId, new BehaviorSubject<ChatMessage[]>([]));
    }
    return this._roomMessages.get(roomId)!.asObservable();
  }

  closeChat(id: number) {
    this._rooms.next(this._rooms.value.filter(r => r.id !== id));
    if (this._currentRoom.value?.id === id) this.clearCurrentRoom();
  }

  closePrivateChat(username: string) {
    this._privateChats.next(this._privateChats.value.filter(u => u.username !== username));
    if (this._currentRoom.value?.username === username) this.clearCurrentRoom();
  }

  clearCurrentRoom() {
    this._currentRoom.next(null);
    this._messages.next([]);
  }
}

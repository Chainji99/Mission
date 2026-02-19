import { Component, inject, OnInit, OnDestroy, ViewChild, ElementRef, AfterViewChecked } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatInputModule } from '@angular/material/input';
import { ChatService, ChatMessage } from '../_services/chat-service';
import { FriendService } from '../_services/friend-service';
import { Subscription } from 'rxjs';

@Component({
    selector: 'app-private-chat',
    standalone: true,
    imports: [CommonModule, FormsModule, MatButtonModule, MatIconModule, MatInputModule],
    templateUrl: './private-chat.html',
    styleUrls: ['./private-chat.scss']
})
export class PrivateChat implements OnInit, OnDestroy, AfterViewChecked {
    private _route = inject(ActivatedRoute);
    private _router = inject(Router);
    private _chat = inject(ChatService);
    private _friends = inject(FriendService);

    username: string = '';
    displayName: string = '';
    input: string = '';
    messages: ChatMessage[] = [];
    private _sub: Subscription | null = null;

    @ViewChild('messagesContainer') private _messagesContainer!: ElementRef;

    ngOnInit() {
        this._route.params.subscribe(params => {
            this.username = params['username'];
            this._loadUserDetails();
            this._subscribeToMessages();
        });
    }

    private _loadUserDetails() {
        const friend = this._friends.friends().find(f => f.username === this.username);
        this.displayName = friend ? friend.display_name : this.username;
    }

    private _subscribeToMessages() {
        if (this._sub) this._sub.unsubscribe();
        this._sub = this._chat.getMessages$(this.username).subscribe(msgs => {
            this.messages = msgs;
            this.scrollToBottom();
        });
    }

    send() {
        if (!this.input.trim()) return;
        this._chat.sendMessage(this.username, this.input.trim());
        this.input = '';
    }

    back() {
        this._router.navigate(['/']);
    }

    ngAfterViewChecked() {
        this.scrollToBottom();
    }

    private scrollToBottom() {
        try {
            const el = this._messagesContainer?.nativeElement;
            if (el) {
                el.scrollTop = el.scrollHeight;
            }
        } catch (err) { }
    }

    ngOnDestroy() {
        if (this._sub) this._sub.unsubscribe();
    }
}

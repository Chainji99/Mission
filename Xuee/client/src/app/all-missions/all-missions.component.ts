import { Component, inject, OnInit, signal, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { MatCardModule } from '@angular/material/card';
import { MatChipsModule } from '@angular/material/chips';
import { RouterModule, Router } from '@angular/router';
import { MissionService } from '../_services/mission-service';
import { ChatService } from '../_services/chat-service';
import { Mission } from '../_models/mission';
import { MissionFilter } from '../_models/mission-filter';
import { animate, style, transition, trigger } from '@angular/animations';

@Component({
  selector: 'app-all-missions',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    MatButtonModule,
    MatIconModule,
    MatFormFieldModule,
    MatInputModule,
    MatSelectModule,
    MatCardModule,
    MatChipsModule,
    RouterModule
  ],
  templateUrl: './all-missions.html',
  styleUrls: ['./all-missions.scss'],
  animations: [
    trigger('fadeInUp', [
      transition(':enter', [
        style({ opacity: 0, transform: 'translateY(20px)' }),
        animate('400ms ease-out', style({ opacity: 1, transform: 'translateY(0)' }))
      ])
    ])
  ]
})
export class AllMissionsComponent implements OnInit {
  private missionService = inject(MissionService);
  private chatService = inject(ChatService);
  private router = inject(Router);

  missions = signal<Mission[]>([]);
  isLoading = signal<boolean>(false);

  filter = signal<MissionFilter>({
    name: '',
    status: ''
  });

  expandedMissionId = signal<number | null>(null);

  ngOnInit() {
    this.loadMissions();
  }

  toggleExpand(missionId: number) {
    if (this.expandedMissionId() === missionId) {
      this.expandedMissionId.set(null);
    } else {
      this.expandedMissionId.set(missionId);
    }
  }

  async loadMissions() {
    this.isLoading.set(true);
    try {
      const data = await this.missionService.get_all(this.filter());
      this.missions.set(data);
    } catch (error) {
      console.error('Failed to load missions', error);
    } finally {
      this.isLoading.set(false);
    }
  }

  onSearch() {
    this.loadMissions();
  }

  getStatusClass(status?: string): string {
    if (!status) return 'status-unknown';
    return `status-${status.toLowerCase()}`;
  }

  async joinMission(mission: Mission) {
    try {
      await this.missionService.join(mission.id, mission);
      // Add to chat history and open chat
      this.chatService.openChat({ id: mission.id, name: mission.name });
      alert(`Successfully joined mission: ${mission.name}`);
      // Navigate to mission chat
      this.router.navigate([`/mission-chat/${mission.id}`]);
      this.loadMissions(); // Refresh to update crew count
    } catch (error) {
      console.error('Failed to join mission', error);
      alert('Failed to join mission. Please try again.');
    }
  }

  openMissionChat(mission: Mission) {
    this.chatService.openChat({ id: mission.id, name: mission.name });
    this.router.navigate([`/mission-chat/${mission.id}`]);
  }
}

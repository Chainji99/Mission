import { Component, inject, OnInit, OnDestroy } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { MissionService } from '../_services/mission-service';
import { ChatService } from '../_services/chat-service';
import { Router, NavigationEnd } from '@angular/router';
import { PassportService } from '../_services/passport-service';
import { Mission } from '../_models/mission';
import { NewMission } from '../_dialogs/new-mission';
import { AddMission } from '../_models/add-mission';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { DatePipe, AsyncPipe } from '@angular/common';
import { BehaviorSubject, filter, Subscription } from 'rxjs';
import { MatTableModule } from '@angular/material/table';

@Component({
  selector: 'app-mission-manager',
  templateUrl: './mission-manager.html',
  styleUrls: ['./mission-manager.scss'],
  imports: [MatButtonModule, MatIconModule, DatePipe, AsyncPipe, MatTableModule]
})
export class MissionManager implements OnInit, OnDestroy {
  private _missionService = inject(MissionService);
  private _passportService = inject(PassportService);
  private _dialog = inject(MatDialog);

  private _missionsSubject = new BehaviorSubject<Mission[]>([]);
  readonly myMissions$ = this._missionsSubject.asObservable();

  displayedColumns: string[] = ['id', 'name', 'mission_date', 'description', 'chief_id', 'crew_count', 'status', 'created_at'];
  totalCrew = 0;

  // Inject chat and router as class fields so click handlers reliably use same instances
  private _chat = inject(ChatService);
  private _router = inject(Router);
  private _routerSubscription: Subscription | null = null;

  async ngOnInit() {
    await this.loadMyMission();
    // Subscribe to route changes to reload data when returning to this page
    this._routerSubscription = this._router.events
      .pipe(filter(event => event instanceof NavigationEnd))
      .subscribe((event: any) => {
        if (event.urlAfterRedirects.includes('/chief')) {
          this.loadMyMission();
        }
      });
  }

  ngOnDestroy() {
    if (this._routerSubscription) {
      this._routerSubscription.unsubscribe();
    }
  }

  async generateTestData() {
    const statuses = ['Open', 'InProgress', 'Completed', 'Fail'];

    // Create 10 missions for each status
    for (const status of statuses) {
      for (let i = 1; i <= 10; i++) {
        const mission: AddMission = {
          name: `Test ${status} ${i}`,
          description: `Auto-generated test mission for status ${status}`,
          status: status
        };

        try {
          await this._missionService.add(mission);
          console.log(`Created ${mission.name}`);
        } catch (e) {
          console.error(`Failed to create ${mission.name}`, e);
        }
      }
    }

    await this.loadMyMission();
    alert('Test data generation completed!');
  }

  async addOneSampleMission() {
    const mission: AddMission = {
      name: 'Sample Mission ' + Math.floor(Math.random() * 1000),
      description: 'This is a sample mission added for testing purposes.',
      status: 'Open',
      mission_date: new Date().toISOString(),
      location: 'Bangkok, Thailand',
      email: 'sample@xue.com',
      phone: '081-123-4567',
      time: '10:00 - 12:00'
    };

    try {
      await this._missionService.add(mission);
      await this.loadMyMission();
      alert('Sample mission added successfully!');
    } catch (e) {
      console.error('Failed to add sample mission', e);
      alert('Failed to add sample mission. Check console for details.');
    }
  }

  formatStatus(status?: string): string {
    if (!status) return 'Unknown';
    switch (status.toLowerCase()) {
      case 'open': return 'Open';
      case 'inprogress': return 'In Progress';
      case 'completed': return 'Completed';
      case 'failed': return 'Failed';
      default: return status;
    }
  }

  private async loadMyMission() {
    try {
      const missions = await this._missionService.getMyMissions();
      this._missionsSubject.next(missions);
      this.totalCrew = await this._missionService.getCrewCount();

      this._chat.setRoomsFromMissions(missions.map(m => ({ id: m.id, name: m.name })));
    } catch (error) {
      console.error('Error loading missions or crew count', error);
    }
  }

  openDialog() {
    const ref = this._dialog.open(NewMission);
    ref.afterClosed().subscribe(async (addMission: AddMission) => {
      if (!addMission) return;
      const id = await this._missionService.add(addMission);
      const now = new Date().toISOString();
      const newMission: Mission = {
        id,
        name: addMission.name,
        description: addMission.description,
        status: 'Open',
        chief_id: 0,
        chief_display_name: this._passportService.data()?.display_name || 'Me',
        crew_count: 0,
        mission_date: addMission.mission_date,
        time: addMission.time,
        email: addMission.email,
        phone: addMission.phone,
        location: addMission.location,
        created_at: now,
        updated_at: now
      };

      const currentMissions = this._missionsSubject.value;
      this._missionsSubject.next([...currentMissions, newMission]);
    })
  }

  openMissionChat(mission: Mission, event?: Event) {
    if (event) event.stopPropagation();
    console.log('openMissionChat called for mission', mission?.id, mission?.name);
    try {
      this._chat.openChat({ id: mission.id, name: mission.name });
      this._router.navigate([`/mission-chat/${mission.id}`]);
    } catch (err) {
      console.error('Failed to open mission chat', err);
    }
  }
}

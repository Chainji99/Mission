import { MissionFilter, MissionStatus } from '../_models/mission-filter';
import { Mission } from '../_models/mission';
import { Component, inject, OnInit, computed } from '@angular/core';
import { Router } from '@angular/router';
import { MissionService } from '../_services/mission-service';
import { PassportService } from '../_services/passport-service';
import { FormsModule } from '@angular/forms';
import { DatePipe, AsyncPipe, CommonModule } from '@angular/common';
import { MatDialog } from '@angular/material/dialog';
import { NewMission } from '../_dialogs/new-mission';
import { AddMission } from '../_models/add-mission';
import { MatTableModule } from '@angular/material/table';
import { BehaviorSubject } from 'rxjs';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { animate, state, style, transition, trigger } from '@angular/animations';

@Component({
    selector: 'app-mission',
    imports: [
        CommonModule,
        FormsModule, 
        DatePipe,
        AsyncPipe,
        MatTableModule, 
        MatFormFieldModule, 
        MatInputModule, 
        MatSelectModule, 
        MatButtonModule,
        MatIconModule,
        MatProgressSpinnerModule
    ],
    templateUrl: './mission.html',
    styleUrls: ['./mission.scss'],
    animations: [
        trigger('detailExpand', [
            state('collapsed,void', style({height: '0px', minHeight: '0'})),
            state('expanded', style({height: '*'})),
            transition('expanded <=> collapsed', animate('225ms cubic-bezier(0.4, 0.0, 0.2, 1)')),
        ]),
    ],
})
export class Missions implements OnInit {
    private _missionService = inject(MissionService);
    private _passportService = inject(PassportService);
    private _dialog = inject(MatDialog);
    private _router = inject(Router);
    
    filter: MissionFilter = {
        status: ''
    }
    
    private _missionsSubject = new BehaviorSubject<Mission[]>([]);
    readonly missions$ = this._missionsSubject.asObservable();
    
    isSignin = this._passportService.isSignin;
    
    displayedColumns = computed(() => {
        return ['name', 'description', 'mission_date', 'chief_display_name', 'crew_count', 'status', 'created_at', 'actions'];
    });

    expandedElement: Mission | null = null;

    isLoading = false;

    constructor() {
        this.filter = this._missionService.filter;
    }

    ngOnInit(): void {
        this.onSubmit();
    }

    async onSubmit() {
        this.isLoading = true;
        try {
            const missions = await this._missionService.get_all(this.filter);
            this._missionsSubject.next(missions);
        } catch (error) {
            console.error('Error loading missions', error);
        } finally {
            this.isLoading = false;
        }
    }

    toggleExpand(element: Mission, event?: Event) {
        if (event) {
            event.stopPropagation();
        }
        this.expandedElement = this.expandedElement === element ? null : element;
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

    async join(mission: Mission, event: Event) {
        event.stopPropagation(); // Prevent row expansion
        if (!confirm(`Are you sure you want to join mission "${mission.name}"?`)) return;
        
        try {
            await this._missionService.join(mission.id, mission);
            alert('Joined mission successfully!');
            this.onSubmit(); // Refresh list
        } catch (error) {
            console.error('Error joining mission', error);
            alert('Failed to join mission.');
        }
    }

    openDialog() {
        if (!this.isSignin()) {
            if (confirm('You must be logged in to create a mission. Do you want to login now?')) {
                this._router.navigate(['/login']);
            }
            return;
        }
        
        const dialogRef = this._dialog.open(NewMission, {
            width: '600px',
            disableClose: true
        });

        dialogRef.afterClosed().subscribe(async (result: AddMission | undefined) => {
            if (result) {
                try {
                    await this._missionService.add(result);
                    this.onSubmit(); // Refresh list
                } catch (error) {
                    console.error('Error creating mission', error);
                }
            }
        });
    }
}

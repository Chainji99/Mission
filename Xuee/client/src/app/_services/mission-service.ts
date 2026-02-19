import { inject, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { firstValueFrom } from 'rxjs';
import { environment } from '../../environments/environment';
import { MissionFilter } from '../_models/mission-filter';
import { Mission } from '../_models/mission';
import { AddMission } from '../_models/add-mission';

@Injectable({
    providedIn: 'root',
})
export class MissionService {
    private _http = inject(HttpClient);
    private _baseUrl = environment.baseUrl + '/api/v1/missions';
    private _apiUrl = environment.baseUrl + '/api/v1';
    private _joinedLocalKey = 'xue_joined_missions';

    filter: MissionFilter = {}

    async get_all(filter: MissionFilter): Promise<Mission[]> {
        try {
            const queryString = this.createQueryString(filter)
            const url = `${this._baseUrl}?${queryString}`;
            const missions = await firstValueFrom(this._http.get<Mission[]>(url));
            return missions;
        } catch (error) {
            console.warn('Server connection failed. Using mock missions for demo.');
            
            const mockMissions: Mission[] = [
                {
                    id: 1,
                    name: 'Protect the Sacred Temple',
                    description: 'Guard the ancient temple from invading forces.',
                    status: 'Open',
                    chief_id: 1,
                    chief_display_name: 'Zen Master',
                    crew_count: 2,
                    crew_names: ['Dragon Spirit', 'Wind Walker'],
                    mission_date: '2025-10-26',
                    time: 'รอผู้สร้างระบุเวลา',
                    email: 'zen.master@example.com',
                    phone: '081-234-5678',
                    location: 'Ancient Temple, Northern Mountain',
                    created_at: new Date().toISOString(),
                    updated_at: new Date().toISOString()
                },
                {
                    id: 2,
                    name: 'Silent Infiltration',
                    description: 'Infiltrate the enemy camp and gather intelligence.',
                    status: 'InProgress',
                    chief_id: 2,
                    chief_display_name: 'Silent Blade',
                    crew_count: 2,
                    crew_names: ['Moon Shadow', 'Fire Fox'],
                    mission_date: '2025-11-05',
                    time: '18:00 - 21:00',
                    email: 'silent.blade@example.com',
                    phone: '089-876-5432',
                    location: 'Enemy Outpost, Shadow Valley',
                    created_at: new Date().toISOString(),
                    updated_at: new Date().toISOString()
                },
                {
                    id: 3,
                    name: 'Dragon Boat Festival Security',
                    description: 'Maintain order and safety during the annual festival.',
                    status: 'Open',
                    chief_id: 3,
                    chief_display_name: 'Fire Fox',
                    crew_count: 5,
                    crew_names: ['Water Dragon', 'Wood Ox'],
                    mission_date: '2025-06-20',
                    time: '08:00 - 18:00',
                    email: 'firefox@temple.org',
                    phone: '085-555-0123',
                    location: 'Riverside Park, East District',
                    created_at: new Date().toISOString(),
                    updated_at: new Date().toISOString()
                },
                {
                    id: 4,
                    name: 'Shadow Intelligence Gathering',
                    description: 'Observe enemy movements near the border without detection.',
                    status: 'InProgress',
                    chief_id: 4,
                    chief_display_name: 'Moon Shadow',
                    crew_count: 3,
                    crew_names: ['Ghost Walker', 'Mist Crawler'],
                    mission_date: '2025-12-12',
                    time: '22:00 - 04:00',
                    email: 'shadow@valley.ninja',
                    phone: '082-999-8888',
                    location: 'Border Outpost, Northern Peaks',
                    created_at: new Date().toISOString(),
                    updated_at: new Date().toISOString()
                },
                {
                    id: 5,
                    name: 'New Sample Mission',
                    description: 'This is a newly added mission to demonstrate the mission board.',
                    status: 'Open',
                    chief_id: 5,
                    chief_display_name: 'New Hero',
                    crew_count: 0,
                    crew_names: [],
                    mission_date: new Date().toISOString(),
                    time: '09:00 - 17:00',
                    email: 'hero@xue.com',
                    phone: '099-000-1111',
                    location: 'Central Plaza, Sky City',
                    created_at: new Date().toISOString(),
                    updated_at: new Date().toISOString()
                }
            ];

            // Apply simple client-side filtering to mock data
            return mockMissions.filter(m => {
                const nameMatch = !filter.name || m.name.toLowerCase().includes(filter.name.toLowerCase());
                const statusMatch = !filter.status || m.status === filter.status;
                return nameMatch && statusMatch;
            });
        }
    }

    async add(mission: AddMission): Promise<number> {
        const url = this._apiUrl + '/mission-management';
        const observable = this._http.post<{ mission_id: number }>(url, mission);
        const resp = await firstValueFrom(observable);
        return resp.mission_id;
    }

    async getMyMissions(): Promise<Mission[]> {
        try {
            // Get missions created by user
            const url = this._apiUrl + '/brawlers/my-missions';
            const myMissions = await firstValueFrom(this._http.get<Mission[]>(url));
            
            // Get missions user joined
            const joinedUrl = this._apiUrl + '/brawlers/joined-missions';
            let joinedMissions: Mission[] = [];
            try {
                joinedMissions = await firstValueFrom(this._http.get<Mission[]>(joinedUrl));
            } catch (e) {
                console.warn('Could not fetch joined missions', e);
            }
            
            // Combine both lists, avoiding duplicates
            const allMissions: Mission[] = [...myMissions];
            const myMissionIds = new Set(myMissions.map(m => m.id));
            
            joinedMissions.forEach(mission => {
                if (!myMissionIds.has(mission.id)) {
                    allMissions.push(mission);
                }
            });

            // Also merge locally joined missions (works when server doesn't support joined endpoint)
            const localJoined = this._readJoinedLocal();
            const existingIds = new Set(allMissions.map(m => m.id));
            localJoined.forEach(m => {
                if (!existingIds.has(m.id)) {
                    allMissions.push(m);
                }
            });
            
            return allMissions;
        } catch (error) {
            console.warn('Server connection failed. Using mock my missions for demo.');
            const base: Mission[] = [
                {
                    id: 101,
                    name: 'My Personal Mission',
                    description: 'This is a mission I created myself.',
                    status: 'Open',
                    chief_id: 1,
                    chief_display_name: 'Me',
                    crew_count: 0,
                    mission_date: new Date().toISOString(),
                    created_at: new Date().toISOString(),
                    updated_at: new Date().toISOString()
                }
            ];
            // Merge local joined into fallback
            const localJoined = this._readJoinedLocal();
            const baseIds = new Set(base.map(m => m.id));
            localJoined.forEach(m => {
                if (!baseIds.has(m.id)) {
                    base.push(m);
                }
            });
            return base;
        }
    }

    async getCrewCount(): Promise<number> {
        const url = this._apiUrl + '/brawlers/crew-count';
        return await firstValueFrom(this._http.get<number>(url));
    }

    async join(missionId: number, mission?: Mission): Promise<void> {
        try {
            const url = `${this._baseUrl}/${missionId}/join`;
            await firstValueFrom(this._http.post(url, {}));
        } catch (error) {
            console.warn('Server join failed. Using mock success for demo.');
            // For demo purposes, we return success even if server fails
        } finally {
            // Persist locally so Mission Manager can show joined missions even offline
            if (mission) {
                const snapshot: Mission = {
                    id: mission.id,
                    name: mission.name,
                    description: mission.description,
                    status: mission.status || 'Open',
                    chief_id: mission.chief_id,
                    chief_display_name: mission.chief_display_name,
                    crew_count: mission.crew_count,
                    mission_date: mission.mission_date,
                    time: mission.time,
                    email: mission.email,
                    phone: mission.phone,
                    location: mission.location,
                    created_at: mission.created_at || new Date().toISOString(),
                    updated_at: mission.updated_at || new Date().toISOString()
                };
                this._saveJoinedLocal(snapshot);
            }
            return Promise.resolve();
        }
    }

    private createQueryString(filter: MissionFilter) {
        this.filter = filter;
        const params: string[] = []
        if(filter.name && filter.name.trim() !== '') {
            params.push(`name=${filter.name}`);
        }
        if(filter.status && filter.status !== '') {
            params.push(`status=${filter.status}`);
        }
        return params.join('&');    
    }

    private _readJoinedLocal(): Mission[] {
        try {
            const raw = localStorage.getItem(this._joinedLocalKey);
            if (!raw) return [];
            const parsed = JSON.parse(raw) as Mission[];
            if (!Array.isArray(parsed)) return [];
            // Deduplicate by id
            const map = new Map<number, Mission>();
            parsed.forEach(m => {
                if (m && typeof m.id === 'number' && !map.has(m.id)) {
                    map.set(m.id, m);
                }
            });
            return Array.from(map.values());
        } catch {
            return [];
        }
    }

    private _saveJoinedLocal(mission: Mission) {
        const list = this._readJoinedLocal();
        const exists = list.find(m => m.id === mission.id);
        if (!exists) {
            list.push(mission);
            localStorage.setItem(this._joinedLocalKey, JSON.stringify(list));
        }
    }

}

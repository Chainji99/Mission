export interface MissionRule {
  id: number;
  title: string;
  description: string;
  icon_url: string;
}

export interface MissionMember {
  id: number;
  display_name: string;
  avatar_url?: string;
  role: 'Chief' | 'Crew';
  info?: string;
  mcode?: string;
}

export interface Mission {
    id: number;
    title?: string;
    name: string;
    chief_id: number;
    chief_display_name: string;
    description?: string;
    crew_count?: number;
    status?: string;
    crew_names?: string[];
    members?: MissionMember[];
    rules?: MissionRule[];
    mission_date?: string;
    time?: string;
    email?: string;
    phone?: string;
    location?: string;
    rewards?: string;
    rewards_tags?: string[];
    created_at?: string;
    updated_at?: string;
}
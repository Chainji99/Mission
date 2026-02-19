export interface FriendUser {
  username: string;
  display_name: string;
  avatar_url?: string;
}

export type FriendStatus = 'Friend' | 'Outgoing' | 'Incoming';

export interface FriendEntry extends FriendUser {
  status: FriendStatus;
}

export interface FriendStore {
  friends: FriendUser[];
  outgoing: FriendUser[];
  incoming: FriendUser[];
}

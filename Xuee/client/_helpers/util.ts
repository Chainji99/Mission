import { Brawler } from '../src/app/_models/brawler';
import { Passport } from '../_models/Passport';
import { PasswordValidator } from './password.validator';

const _default_avatar_url = 'assets/default_avatar.png';

export function getAvatarUrl(avatar_url: string | undefined): string {
  return avatar_url || _default_avatar_url;
}  
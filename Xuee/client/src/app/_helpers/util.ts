import { Passport } from '../_models/Passport';

export function getAvatarUrl(passport?: Passport): string {
  return passport?.avatar_url || 'assets/anonymous_128.png';
}
import { Routes } from '@angular/router';
import { Home } from './home/home';
import { Login } from './login/login';
import { Profile } from './profile/profile';
import { Missions } from './mission/mission';
import { ServerError } from './server-error/server-error';
import { NotFound } from './not-found/not-found';
import { MissionManager } from './mission-manager/mission-manager';
import { authGuard } from './_guard/auth-guard';
import { GoogleCallbackComponent } from './google-callback/google-callback.component';
import { CardGalleryComponent } from './card-gallery/card-gallery.component';
import { GachaComponent } from './gacha/gacha.component';
import { InventoryComponent } from './inventory/inventory.component';


export const routes: Routes = [
  { path: '', component: Home },
  { path: 'login', component: Login },
  { path: 'reset-password', loadComponent: () => import('./reset-password/reset-password').then(m => m.ResetPassword) },
  { path: 'google-callback', component: GoogleCallbackComponent },
  { path: 'cards', loadComponent: () => import('./card-gallery/card-gallery.component').then(m => m.CardGalleryComponent) },
  { path: 'gacha', loadComponent: () => import('./gacha/gacha.component').then(m => m.GachaComponent) },

  { path: 'inventory', loadComponent: () => import('./inventory/inventory.component').then(m => m.InventoryComponent), canActivate: [authGuard] },
  { path: 'profile', component: Profile },
  { path: 'all-missions', loadComponent: () => import('./all-missions/all-missions.component').then(m => m.AllMissionsComponent) },
  { path: 'missions', component: Missions },
  {
    path: 'chief',
    loadComponent: () => import('./mission-manager/mission-manager').then(m => m.MissionManager),
    runGuardsAndResolvers: 'always',
    canActivate: [authGuard]
  },
  { path: 'mission-chat/:id', loadComponent: () => import('./mission-chat/mission-chat').then(m => m.MissionChat) },
  { path: 'private-chat/:username', loadComponent: () => import('./private-chat/private-chat').then(m => m.PrivateChat) },
  { path: 'server-error', component: ServerError },
  { path: '**', component: NotFound },
];




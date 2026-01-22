
import { ApplicationConfig, importProvidersFrom, provideBrowserGlobalErrorListeners } from '@angular/core';
import { provideRouter } from '@angular/router';
import { routes } from './src/app/app.routes';
import { provideHttpClient, withInterceptors } from '@angular/common/http';
import { NgxSpinnerModule } from "ngx-spinner";
import { loadingInterceptor } from './src/app/_intercaptors/loading-interceptor';
import { errorInterceptor } from './src/app/_intercaptors/error-interceptor';


export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideRouter(routes),
    provideHttpClient(withInterceptors ([loadingInterceptor, errorInterceptor])),
    importProvidersFrom(NgxSpinnerModule)
  ]
};
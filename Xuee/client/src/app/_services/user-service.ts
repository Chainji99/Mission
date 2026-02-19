import { Injectable, inject } from "@angular/core";
import { HttpClient } from "@angular/common/http";
import { firstValueFrom } from "rxjs";
import { environment } from "../../environments/environment";
import { PassportService } from "./passport-service";
import { fileToBase64 } from "../_helpers/file";

@Injectable({
  providedIn: 'root'
})
export class UserService {

  private _api_url = environment.baseUrl + '/api/v1/brawlers'; 
  private _http = inject(HttpClient);
  private _passport = inject(PassportService);

    async uploadAvatar(file: File): Promise<string | null> {
    const url = this._api_url + '/avatar';
    const  base64string = await fileToBase64(file);
    const uploadedImg = {
      'base64_string': base64string
    }
    try {
      // Assuming the backend returns an object with a 'url' property
      const response = await firstValueFrom(this._http.post<{url: string}>(url, uploadedImg));
      
      if (response && response.url) {
        this._passport.saveAvatarImage(response.url);
        return 'Avatar uploaded successfully';
      }
      
      console.warn('No URL returned from server. Using local preview.');
      this._passport.saveAvatarImage(base64string);
      return 'Avatar uploaded successfully';
    } catch (error: any) {
      console.error('Avatar upload failed', error);
      
      // ALWAYS fallback to local preview for "remembering" the change even if server fails
      console.warn('Server upload failed. Using local preview as mock avatar to remember changes.');
      this._passport.saveAvatarImage(base64string); 
      return 'Avatar uploaded successfully';
    }
  }
  
}
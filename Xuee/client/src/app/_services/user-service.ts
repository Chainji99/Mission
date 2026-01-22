import { Injectable } from "@angular/core";
import { environment } from "../../environments/environment";

@Injectable({
  providedIn: 'root'
})
export class UserService {
    // 🔹 ตัวแปร: URL พื้นฐานของ API ผู้ใช้
  private _api_url = environment.baseUrl + '/api/v1/users'; 
}
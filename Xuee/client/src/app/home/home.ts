import { Component, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { MatButtonModule } from '@angular/material/button';
import { RouterModule } from '@angular/router';
import { environment } from '../../environments/environment';

@Component({
  selector: 'app-home',
  templateUrl: './home.html',
  styleUrls: ['./home.scss'],
  imports: [MatButtonModule, RouterModule]
})
export class Home {
  private _http = inject(HttpClient);
  private _baseUrl = environment.baseUrl;

  get500Error() {
    this._http.get(this._baseUrl + '/api/buggy/server-error').subscribe({
      next: response => console.log(response),
      error: error => console.log(error)
    })
  }
}
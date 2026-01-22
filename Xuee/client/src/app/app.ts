import { Component, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { Navbar } from "./navbar/navbar";
import { NgxSpinnerModule } from "ngx-spinner";


@Component({
  selector: 'app-root',
  imports : [RouterOutlet, Navbar, NgxSpinnerModule],
  templateUrl: './app.html',
  styleUrl: './app.scss'
})
export class App {
  protected readonly title = signal('client');
}

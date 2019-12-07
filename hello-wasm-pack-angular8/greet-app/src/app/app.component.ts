import { Component, OnInit } from '@angular/core';
import * as wasm from 'hello-wasm-pack';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {
  title = 'greet-app';

    ngOnInit(): void {
      wasm.greet();
  }  
}

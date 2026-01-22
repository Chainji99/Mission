import {inject, Injectable} from '@angular/core';
import { NgxSpinnerService } from "ngx-spinner";


@Injectable({
    providedIn: 'root'
})
export class Loading{
    loadingRequestsCount = 0;
    private _spinner = inject(NgxSpinnerService);

    loading(){
        this.loadingRequestsCount++;
        this._spinner.show(undefined, {
            type: "ball_8bits",
            bdColor: "rgba(0,0,0,0.5)",
            color: "#fff",
            fullScreen: true
        })
    }    

    idle(){
        this.loadingRequestsCount--
        if(this.loadingRequestsCount <= 0){
            this.loadingRequestsCount = 0;
            this._spinner.hide();
        }
    }
}
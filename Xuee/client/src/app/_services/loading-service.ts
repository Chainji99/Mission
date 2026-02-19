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
        // Debounce spinner to show only for slow requests (> 300ms)
        setTimeout(() => {
            if (this.loadingRequestsCount > 0) {
                this._spinner.show(undefined, {
                    type: "ball_8bits",
                    bdColor: "rgba(0,0,0,0.5)",
                    color: "#fff",
                    fullScreen: true
                });
            }
        }, 300);
    }    

    idle(){
        this.loadingRequestsCount--
        if(this.loadingRequestsCount <= 0){
            this.loadingRequestsCount = 0;
            this._spinner.hide();
        }
    }
}
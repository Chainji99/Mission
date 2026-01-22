import { HttpInterceptorFn } from "@angular/common/http";
import { inject } from "@angular/core";
import { Loading } from "../_services/loading-service";
import { finalize } from "rxjs/operators";


export const loadingInterceptor: HttpInterceptorFn = (req, next) => {
    const loadingService = inject(Loading);
    loadingService.loading();
    return next(req).pipe(
        finalize(() => loadingService.idle())
    );
};
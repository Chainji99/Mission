import { Component, inject } from '@angular/core';
import { MatDialogRef, MatDialogContent, MatDialogActions, MatDialogTitle, MatDialogClose } from '@angular/material/dialog';
import { AddMission } from '../_models/add-mission';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';

@Component({
    selector: 'app-new-mission',
    templateUrl: './new-mission.html',
    styleUrls: ['./new-mission.scss'],
    imports: [FormsModule, MatDialogContent, MatDialogActions, MatDialogTitle, MatButtonModule, MatInputModule, MatFormFieldModule, MatDialogClose]
})
export class NewMission {
    addMission: AddMission = {
        name: '',
        description: '',
        mission_date: '',
        time: '',
        email: '',
        phone: '',
        location: ''
    }
    private readonly _dialogRef = inject(MatDialogRef<NewMission>)

    onSubmit() {
        const mission = this.clean(this.addMission)
        this._dialogRef.close(mission)
    }

    private clean(addMission: AddMission): AddMission {
        let missionDate: string | undefined = undefined;
        if (addMission.mission_date) {
            // Convert YYYY-MM-DD to YYYY-MM-DDT00:00:00 for NaiveDateTime
            missionDate = `${addMission.mission_date}T00:00:00`;
        }

        return {
            name: addMission.name.trim() || 'untitle',
            description: addMission.description?.trim() || undefined,
            mission_date: missionDate,
            time: addMission.time?.trim() || undefined,
            email: addMission.email?.trim() || undefined,
            phone: addMission.phone?.trim() || undefined,
            location: addMission.location?.trim() || undefined
        }
    }
}
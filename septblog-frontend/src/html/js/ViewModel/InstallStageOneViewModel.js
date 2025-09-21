'use strict';

import { HttpError } from "../../admin/js/Errors/HttpError.js";
import { InternalServerError } from "../../admin/js/Errors/InternalServerError.js";
import { InstallStageOneModel } from "../Model/InstallStageOneModel.js";

export class InstallStageOneViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new InstallStageOneModel(this.#config);
    }

    async alreadyInstalled() {
        let res = null;

        try {
            res = await this.model.alreadyInstalled();
        } catch(error) {
            throw error;
        }

        if (res.status === 200) {
            const resJson = await res.json();

            return resJson;
        } else if (res.status === 500) {
            const resJson = await res.json();
            
            throw new InternalServerError(resJson.error_message);
        } else {
            throw new HttpError("HTTP error. " + res.status + ".");
        }
    }
}
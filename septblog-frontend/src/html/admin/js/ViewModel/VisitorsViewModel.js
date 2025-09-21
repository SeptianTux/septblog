'use strict';

import { HttpError } from "../Errors/HttpError.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { VisitorsModel } from "../Model/VisitorsModel.js";

export class VisitorsViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new VisitorsModel(this.#config);
    }

    async getVisitorsDataFromBackend(page) {
        let res = null;

        try {
            res = await this.model.getVisitorsDataFromBackend(page);
        } catch(error) {
            throw error;
        }

        if (res.status === 200) {
            const resJson = await res.json();

            return resJson;
        } else if (res.status === 401) {
            throw new UnauthorizedError("Unauthorized.");
        } else {
            throw new HttpError("HTTP error. " + res.status + ".");
        }
    }
}
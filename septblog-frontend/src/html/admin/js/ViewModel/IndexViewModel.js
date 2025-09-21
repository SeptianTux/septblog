'use strict';

import { HttpError } from "../Errors/HttpError.js";
import { IndexModel } from "../Model/IndexModel.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";

export class IndexViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new IndexModel(this.#config);
    }

    async getVisitorForChart(start, end) {
        let res = null;

        try {
            res = await this.model.getVisitorForChart(start, end);
        } catch(error) {
            throw error;
        }

        if (res.ok) {
            const resJson = await res.json();

            return resJson;
        } else {
            if (res.response === 401) {
                throw new UnauthorizedError("Unauthorized");
            } else {
                throw new HttpError("HTTP error. " + res.status + ".");
            }
        }
    }

    async getVisitorForTable() {
        let res = null;

        try {
            res = await this.model.getVisitorForTable();
        } catch(error) {
            throw error;
        }

        if (res.ok) {
            const resJson = await res.json();

            return resJson;
        } else {
            if (res.response === 401) {
                throw new UnauthorizedError("Unauthorized");
            } else {
                throw new HttpError("HTTP error. " + res.status + ".");
            }
        }
    }
}
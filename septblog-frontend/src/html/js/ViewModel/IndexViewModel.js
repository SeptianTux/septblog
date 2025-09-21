'use strict';

import { HttpError } from "../../admin/js/Errors/HttpError.js";
import { IndexModel } from "../Model/IndexModel.js";

export class IndexViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new IndexModel(this.#config);
    }

    async getArticlesFromBackend(page) {
        let res = null;

        try {
            res = await this.model.getArticlesFromBackend(page);
        } catch(error) {
            throw error;
        }

        if (res.status === 200) {
            const resJson = await res.json();

            return resJson;
        } else if (res.status === 500) {
            const resJson = await res.json();

            throw new HttpError(resJson.error_message);
        } else {
            throw new HttpError("HTTP error. " + res.status + ".");
        }
    }
}
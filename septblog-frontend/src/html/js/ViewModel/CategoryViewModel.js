'use strict';

import { HttpError } from "../../admin/js/Errors/HttpError.js";
import { NotFoundError } from "../../admin/js/Errors/NotFoundError.js";
import { CategoryModel } from "../Model/CategoryModel.js";

export class CategoryViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new CategoryModel(this.#config);
    }

    async getArticlesFromBackend(categoryName, page) {
        let res = null;

        try {
            res = await this.model.getArticlesFromBackend(categoryName, page);
        } catch(error) {
            throw error;
        }

        if (res.status === 200) {
            const resJson = await res.json();

            return resJson;
        } else if (res.status === 404) {
            throw new NotFoundError("Not found.");
        } else if (res.status === 500) {
            throw new HttpError("HTTP error. " + res.status + ".");
        } else {
            throw new HttpError("HTTP error. " + res.status + ".");
        }
    }
}
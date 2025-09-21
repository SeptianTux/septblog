'use strict';

import { HttpError } from "../../admin/js/Errors/HttpError.js";
import { NotFoundError } from "../../admin/js/Errors/NotFoundError.js";
import { ArticleModel } from "../Model/ArticleModel.js";

export class ArticleViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new ArticleModel(this.#config);

        this.getArticleFromBackend('ATFfuYTf');
    }

    async getArticleFromBackend(id) {
        let res = null;

        try {
            res = await this.model.getArticleFromBackend(id);
        } catch(error) {
            throw error;
        }

        if (res.status === 200) {
            const resJson = await res.json();

            return resJson;
        } else if (res.status === 404) {
            throw new NotFoundError("Not found.");
        } else {
            throw new HttpError("HTTP error. " + res.status + ".");
        }
    }
}
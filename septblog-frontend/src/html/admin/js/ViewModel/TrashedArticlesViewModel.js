'use strict';

import { HttpError } from "../Errors/HttpError.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { TrashedArticlesModel } from "../Model/TrashedArticlesModel.js";

export class TrashedArticlesViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new TrashedArticlesModel(this.#config);
    }

    async getArticles(page) {
        let res = null;

        try {
            res = await this.model.getArticles(page);
        } catch(error) {
            throw error;
        }

        if (res.status === 200) {
            const resJson = await res.json();
            return resJson;
        } else if (res.status === 401) {
            throw new UnauthorizedError("Unauthorized.");
        } else if (res.status === 500) {
            const resJson = await res.json();
            throw new HttpError("HTTP error. " + resJson.error_message);
        } else {
            throw new HttpError("HTTP error. " + res.status);
        }
    }

    async deleteArticle(articleId) {
        let res = null;

        try {
            res = await this.model.deleteArticle(articleId);
        } catch(error) {
            throw error;
        }

        if (res.status === 200) {
            const resJson = await res.json();
            return resJson;
        } else if (res.status === 500) {
            const resJson = await res.json();
            throw new HttpError("Internal server error. " + resJson.error_message + ".");
        } else if (res.stauts === 401) {
            throw new UnauthorizedError("Unauthorized.");
        } else {
            throw new HttpError("HTTP error." + res.status + ".");
        }
    }
}
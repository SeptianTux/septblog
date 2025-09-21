'use strict';

import { HttpError } from "../Errors/HttpError.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { ArticlesModel } from "../Model/ArticlesModel.js";

export class ArticlesViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new ArticlesModel(this.#config);
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

    async moveArticleToTrash(articleId) {
        let res = null;

        try {
            res = await this.model.moveArticleToTrash(articleId);
        } catch(error) {
            throw error;
        }

        if (res.status === 200) {
            const resJson = await res.json();
            return resJson;
        } else if (res.status === 500) {
            const resJson = await res.json();

            throw new HttpError("HTTP error. " + resJson.error_message);
        } else if (res.status === 401) {
            throw new UnauthorizedError("Unauthorized.");
        } else {
            throw new HttpError("HTTP error. " + res.status);
        }
    }
}
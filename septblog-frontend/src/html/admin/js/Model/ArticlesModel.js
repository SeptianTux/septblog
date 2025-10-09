'use strict';

import { UnauthorizedError } from "../Errors/UnauthorizedError.js";

export class ArticlesModel {
    #config = null;

    constructor(config) {
        this.#config = config;
    }

    async fetchDataFromBackend(url, method) {
        const accessToken = localStorage.getItem("access_token");
        let get = null;

        if (!accessToken) {
            throw new UnauthorizedError("Invalid credentials.");
        }

        try {
            get = await fetch(url, {
                method: method,
                headers: {
                    'Authorization': `Bearer ${accessToken}`
                }
            });
        } catch(error) {
            throw error;
        }

        return get;
    }

    async getArticles(page) {
        let res = null;

        try {
            const url = this.#config.backendUrl + "/admin/articles/" + page;
            res = await this.fetchDataFromBackend(url, "GET");
        } catch(error) {
            throw error;
        }

        return res;
    }

    async moveArticleToTrash(articleId) {
        let res = null;

        try {
            const url = this.#config.backendUrl + "/admin/articles/move-to-trash/" + articleId;
            res = await this.fetchDataFromBackend(url, "GET");
        } catch(error) {
            throw error;
        }

        return res;
    }
}
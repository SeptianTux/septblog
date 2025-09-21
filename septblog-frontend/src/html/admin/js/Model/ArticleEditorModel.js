'use strict';

import { UnauthorizedError } from "../Errors/UnauthorizedError.js"

export class ArticleEditorModel {
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

    async putDataToBackend(url, body) {
        const accessToken = localStorage.getItem("access_token");
        let res = null;

        if (!accessToken) {
            throw new UnauthorizedError("Invalid credentials.");
        }

        try {
            res = await fetch(url, {
                method: "PUT",
                credentials: "include",
                headers: {
                    "Content-Type": "application/json",
                    'Authorization': `Bearer ${accessToken}`
                },
                body: body
            });
        } catch(error) {
            throw error;
        }

        return res;
    }

    async getArticleCategoriesFromBackend() {
        let res = null;

        try {
            const url = this.#config.backendProtocol + this.#config.backendHost + ":" + this.#config.backendPort + "/admin/article-editor/get-article-categories";
            res = await this.fetchDataFromBackend(url, "GET");
        } catch(error) {
            throw error;
        }

        return res;
    }

    async getArticleFromBackend(articleId) {
        let res = null;

        try {
            const url = this.#config.backendProtocol + this.#config.backendHost + ":" + this.#config.backendPort + "/admin/article-editor/get-article/" + articleId;
            res = await this.fetchDataFromBackend(url, "GET");
        } catch(error) {
            throw error;
        }

        return res;
    }

    async saveArticleToDraft(
        articleId,
        articleTitle,
        articleContent,
        articleCategories
    ) {
        const body = JSON.stringify({
            article_id: articleId,
            article_title: articleTitle,
            article_content: articleContent,
            article_categories: articleCategories,
            article_status: 1
        });

        let res = null;

        try {
            const url = this.#config.backendProtocol + this.#config.backendHost + ":" + this.#config.backendPort + "/admin/article-editor"; 
            res = this.putDataToBackend(url, body);
        } catch(error) {
            throw error;
        }

        return res;
    }

    async publishArticle(
        articleId,
        articleTitle,
        articleContent,
        articleCategories
    ) {
        const body = JSON.stringify({
            article_id: articleId,
            article_title: articleTitle,
            article_content: articleContent,
            article_categories: articleCategories,
            article_status: 2
        });

        let res = null;

        try {
            const url = this.#config.backendProtocol + this.#config.backendHost + ":" + this.#config.backendPort + "/admin/article-editor"; 
            res = this.putDataToBackend(url, body);
        } catch(error) {
            throw error;
        }

        return res;
    }

    async saveArticle(
        articleId,
        articleTitle,
        articleContent,
        articleCategories
    ) {
        const body = JSON.stringify({
            article_id: articleId,
            article_title: articleTitle,
            article_content: articleContent,
            article_categories: articleCategories,
            article_status: 2
        });

        let res = null;

        try {
            const url = this.#config.backendProtocol + this.#config.backendHost + ":" + this.#config.backendPort + "/admin/article-editor"; 
            res = this.putDataToBackend(url, body);
        } catch(error) {
            throw error;
        }

        return res;
    }

    async articleEditorImageUpload(xhr) {
        const accessToken = localStorage.getItem("access_token");

        if (!accessToken) {
            throw new UnauthorizedError("Unauthorized.");
        }

        const url = this.#config.backendProtocol + this.#config.backendHost + ":" + this.#config.backendPort + "/admin/image-upload";

        xhr.open("PUT", url, true);
        xhr.setRequestHeader("Authorization", "Bearer " + accessToken);
    }
}
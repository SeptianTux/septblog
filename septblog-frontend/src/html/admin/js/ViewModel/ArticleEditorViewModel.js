'use strict';

import { ArticleEditorModel } from "../Model/ArticleEditorModel.js";
import { User } from "../Utils/User.js";
import { HttpError } from "../Errors/HttpError.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { BadRequestError } from "../Errors/BadRequestError.js";
import { InternalServerError } from "../Errors/InternalServerError.js";

export class ArticleEditorViewModel {
    model = null;
    #config = null;

    constructor(config) {
        this.#config = config;
        this.model = new ArticleEditorModel(this.#config);
    }

    async checkCredentials() {
        let credentials = false;

        try {
            credentials = await User.checkCredentials();
        } catch(error) {
            throw error;
        }

        return credentials;
    }

    async getArticleCategoriesFromBackend() {
        let res = null;

        try {
            res = await this.model.getArticleCategoriesFromBackend();
        } catch(error) {
            throw error;
        }

        if (!res.ok) {
            if (res.status === 401) {
                throw new UnauthorizedError("Unauthorized.");
            } else {
                throw new HttpError("Http error. Fetch return " + res.status + " code.");
            }
        }

        const resJson = await res.json();

        return resJson.data;
    }

    async getArticleFromBackend(articleId) {
        let res = null;

        try {
            res = await this.model.getArticleFromBackend(articleId);
        } catch(error) {
            throw error;
        }

        if (res.status === 200) {
            const resJson = await res.json();
            return resJson;
        } else if (res.status === 404) {
            throw new Error("Article not found.");
        } else if (res.status === 403) {
            throw new Error("You don't have credentials to get or edit this article.");
        } else if (res.status === 500) {
            const resJson = await res.json();
            throw new Error("Error code : " + resJson.error_code + ". Error message : " + resJson.error_message + ".");
        } else {
            throw new HttpError("HTTP error. " + res.status);
        }
    }

    async saveArticleToDraft(
        articleId,
        articleTitle,
        articleContent,
        articleCategories
    ) {
        let res = null;

        try {
            res = await this.model.saveArticleToDraft(
                articleId,
                articleTitle,
                articleContent,
                articleCategories
            );
        } catch(error) {
            throw error;
        }
        
        if (res.status !== 200) {
            if (res.status === 401) {
                throw new UnauthorizedError("Unauthorized.");
            } else if(res.status === 400) {
                const resJson = await res.json();

                throw new BadRequestError(resJson.error_message);
            } else if(res.status === 500) {
                const resJson = await res.json();

                throw new InternalServerError(resJson.error_message);
            }
            else {
                throw new HttpError("HTTP error. The backend return " + res.status + ".");
            }
        }

        const resJson = await res.json();

        return resJson;
    }

    async publishArticle(
        articleId,
        articleTitle,
        articleContent,
        articleCategories
    ) {
        let res = null;

        try {
            res = await this.model.publishArticle(
                articleId,
                articleTitle,
                articleContent,
                articleCategories
            );
        } catch(error) {
            throw error;
        }
        
        if (res.status !== 200) {
            if (res.status === 401) {
                throw new UnauthorizedError("Unauthorized.");
            } else if(res.status === 400) {
                const resJson = await res.json();

                throw new BadRequestError(resJson.error_message);
            } else if(res.status === 500) {
                const resJson = await res.json();

                throw new InternalServerError(resJson.error_message);
            }
            else {
                throw new HttpError("HTTP error. The backend return " + res.status + ".");
            }
        }

        const resJson = await res.json();

        return resJson;
    }

    async saveArticle(
        articleId,
        articleTitle,
        articleContent,
        articleCategories
    ) {
        let res = null;

        try {
            res = await this.model.saveArticle(
                articleId,
                articleTitle,
                articleContent,
                articleCategories
            );
        } catch(error) {
            throw error;
        }
        
        if (res.status !== 200) {
            if (res.status === 401) {
                throw new UnauthorizedError("Unauthorized.");
            } else if(res.status === 400) {
                const resJson = await res.json();

                throw new BadRequestError(resJson.error_message);
            } else if(res.status === 500) {
                const resJson = await res.json();

                throw new InternalServerError(resJson.error_message);
            }
            else {
                throw new HttpError("HTTP error. The backend return " + res.status + ".");
            }
        }

        const resJson = await res.json();

        return resJson;
    }

    async articleEditorImageUpload(xhr) {
        let res = null;

        try {
            res = this.model.articleEditorImageUpload(xhr);
        } catch(error) {
            throw error;
        }
    }
}
'use strict';

export class CategoryModel {
    #config = null;

    constructor(config) {
        this.#config = config;
    }

    async getDataFromBackend(url) {
        let get = null;

        try {
            get = await fetch(url, {
                method: "GET"
            });
        } catch(error) {
            throw error;
        }

        return get;
    }

    async getArticlesFromBackend(categoryName, page) {
        let res = null;

        if (page === NaN) {
            page = 1;
        } else if (page <= 0) {
            page = 1;
        }

        try {
            const url = this.#config.backendUrl + "/category/" + categoryName + "/" + page;

            res = await this.getDataFromBackend(url);
        } catch(error) {
            throw error;
        }

        return res;
    }
}
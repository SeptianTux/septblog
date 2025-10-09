'use strict';

import { UnauthorizedError } from "../Errors/UnauthorizedError.js";

export class VisitorsModel {
    #config = null;

    constructor(config) {
        this.#config = config;
    }

    async getDataFromBackend(url) {
        const accessToken = localStorage.getItem("access_token");
        let get = null;

        if (!accessToken) {
            throw new UnauthorizedError("Invalid credentials.");
        }

        try {
            get = await fetch(url, {
                method: "GET",
                headers: {
                    'Authorization': `Bearer ${accessToken}`
                }
            });
        } catch(error) {
            throw error;
        }

        return get;
    }

    async getVisitorsDataFromBackend(page) {
        let res = null;

        if (page === NaN) {
            page = 1;
        } else if (page <= 0) {
            page = 1;
        }

        try {
            const url = this.#config.backendUrl + "/admin/visitors/" + page;
            res = await this.getDataFromBackend(url);
        } catch(error) {
            throw error;
        }

        return res;
    }
}
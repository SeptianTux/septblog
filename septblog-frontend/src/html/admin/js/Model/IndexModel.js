'use strict';

import { UnauthorizedError } from "../Errors/UnauthorizedError.js";

export class IndexModel {
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

    async getVisitorForChart(start, end) {
        let res = null;

        try {
            const url = this.#config.backendUrl + "/admin/dashboard/chart/" + start + "/" + end;
            res = await this.fetchDataFromBackend(url, "GET");
        } catch(error) {
            throw error;
        }

        return res;
    }

    async getVisitorForTable() {
        let res = null;

        try {
            const url = this.#config.backendUrl + "/admin/dashboard/visitors";
            res = await this.fetchDataFromBackend(url, "GET");
        } catch(error) {
            throw error;
        }

        return res;
    }
}
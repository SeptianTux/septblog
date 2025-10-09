'use strict';

export class UsersModel {
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

    async putDataToBackend(url) {
        const accessToken = localStorage.getItem("access_token");
        let get = null;

        if (!accessToken) {
            throw new UnauthorizedError("Invalid credentials.");
        }

        try {
            get = await fetch(url, {
                method: "PUT",
                headers: {
                    'Authorization': `Bearer ${accessToken}`
                }
            });
        } catch(error) {
            throw error;
        }

        return get;
    }

    async getUsersDataFromBackend(page) {
        let res = null;

        if (page === NaN) {
            page = 1;
        } else if (page <= 0) {
            page = 1;
        }

        try {
            const url = this.#config.backendUrl + "/admin/users/" + page;
            res = await this.getDataFromBackend(url);
        } catch(error) {
            throw error;
        }

        return res;
    }

    async activateUser(userId) {
        let res = null;

        try {
            const url = this.#config.backendUrl + "/admin/users/1/" + userId;
            res = await this.putDataToBackend(url);
        } catch(error) {
            throw error;
        }

        return res;
    }

    async suspendUser(userId) {
        let res = null;

        try {
            const url = this.#config.backendUrl + "/admin/users/2/" + userId;
            res = await this.putDataToBackend(url);
        } catch(error) {
            throw error;
        }

        return res;
    }

    async deleteUser(userId) {
        let res = null;

        try {
            const url = this.#config.backendUrl + "/admin/users/3/" + userId;
            res = await this.putDataToBackend(url);
        } catch(error) {
            throw error;
        }

        return res;
    }

    async getUserLevel() {
        let res = null;

        try {
            const url = this.#config.backendUrl + "/admin/get-user-level";
            res = await this.getDataFromBackend(url);
        } catch(error) {
            throw error;
        }

        return res;
    }
}
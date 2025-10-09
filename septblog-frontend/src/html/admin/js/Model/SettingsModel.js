'use strict';

export class SettingsModel {
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

    async putDataToBackend(url, body) {
        const accessToken = localStorage.getItem("access_token");
        let res = null;

        if (!accessToken) {
            throw new UnauthorizedError("Invalid credentials.");
        }

        try {
            res = await fetch(url, {
                method: "PUT",
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

    async getSettingsDataFromBackend() {
        let res = null;

        try {
            const url = this.#config.backendUrl + "/admin/settings";
            res = await this.getDataFromBackend(url);
        } catch(error) {
            throw error;
        }

        return res;
    }

    async putSettingsDataToBackend(
        siteTitle,
        tagline,
        signupPage
    ) {
        let res = null;
        const body = JSON.stringify({
            site_title: siteTitle,
            site_tagline: tagline,
            enable_signup_page: signupPage
        });

        try {
            const url = this.#config.backendUrl + "/admin/settings";
            res = await this.putDataToBackend(url, body);
        } catch(error) {
            throw error;
        }

        return res;
    }
}
'use strict';

export class ProfileModel {
    #config = null;

    constructor(config) {
        this.#config = config;
    }

    async fetchDataFromBackend(url) {
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

    async getProfileData() {
        let res = null;

        try {
            const url = this.#config.backendProtocol + this.#config.backendHost + ":" + this.#config.backendPort + "/admin/profile";
            res = await this.fetchDataFromBackend(url);
        } catch(error) {
            throw error;
        }

        return res;
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

    async putProfileData(
        avatar,
        firstName,
        lastName,
        about
    ) {
        const body = JSON.stringify({
            avatar: avatar,
            first_name: firstName,
            last_name: lastName,
            about: about
        });
        let res = null;

        try {
            const url = this.#config.backendProtocol + this.#config.backendHost + ":" + this.#config.backendPort + "/admin/profile";
            res = await this.putDataToBackend(url, body);
        } catch(error) {
            throw error;
        }

        return res;
    }

    async imageUpload(xhr) {
        const accessToken = localStorage.getItem("access_token");

        if (!accessToken) {
            throw new UnauthorizedError("Unauthorized.");
        }

        const url = this.#config.backendProtocol + this.#config.backendHost + ":" + this.#config.backendPort + "/admin/image-upload";

        xhr.open("PUT", url, true);
        xhr.setRequestHeader("Authorization", "Bearer " + accessToken);
    }
}
'use strict';

export class UserModel {
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

    async getProfileDataFromBackend(username) {
        let res = null;

        try {
            const url = this.#config.backendProtocol + this.#config.backendHost + ":" + this.#config.backendPort + "/user/" + username;
            res = await this.getDataFromBackend(url);
        } catch(error) {
            throw error;
        }

        return res;
    }
}
'use strict';

export class InstallStageTwoModel {
    #config = null;

    constructor(config) {
        this.#config = config;
    }

    async putDataToBackend(
        siteTitle,
        tagline,
        firstName,
        lastName,
        username,
        email,
        password1,
        password2
    ) {
        let res = null;

        try {
            const url = this.#config.backendProtocol + this.#config.backendHost + ":" + this.#config.backendPort + "/install";

            res = await fetch(url, {
                method: "PUT",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    site_title: siteTitle,
                    tagline: tagline,
                    first_name: firstName,
                    last_name: lastName,
                    username: username,
                    email: email,
                    password1: password1,
                    password2: password2
                })
            });
        } catch(error) {
            throw error;
        }

        return res;
    }

    async alreadyInstalled() {
        let get = null;

        try {
            const url = this.#config.backendProtocol + this.#config.backendHost + ":" + this.#config.backendPort + "/already-installed";
            
            get = await fetch(url, {
                method: "GET"
            });
        } catch(error) {
            throw error;
        }

        return get;
    }
}
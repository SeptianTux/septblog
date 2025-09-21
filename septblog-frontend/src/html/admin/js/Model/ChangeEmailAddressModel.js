'use strict';

export class ChangeEmailAddressModel {
    #config = null;

    constructor(config) {
        this.#config = config;
    }

    async putEmailToBackend(newEailAddress) {
        const accessToken = localStorage.getItem("access_token");
        let res = null;

        if (!accessToken) {
            throw new UnauthorizedError("Invalid credentials.");
        }

        const body = JSON.stringify({
            new_email_address: newEailAddress
        });

        const url = this.#config.backendProtocol + this.#config.backendHost + ":" + this.#config.backendPort + "/admin/security/change-email"; 

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
}
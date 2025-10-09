'use strict';

export class SignupModel {
    #config = null;

    constructor(config) {
        this.#config = config;
    }

    async signup(
        firstName,
        lastName,
        username,
        email,
        password1,
        password2
    ) {
        let res = null;

        try {
            const url = this.#config.backendUrl + "/admin/signup";

            res = await fetch(url, {
                method: "PUT",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
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
}
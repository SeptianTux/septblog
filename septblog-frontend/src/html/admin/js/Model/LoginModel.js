'use strict';

import { HttpError  } from "../Errors/HttpError.js";

export class LoginModel {
    config = null;
    constructor(config) {
        this.config = config;
    }

    async login(email, password) {
        let res = null;
        
        try {
            const url = this.config.backendProtocol + this.config.backendHost + ":" + this.config.backendPort + "/admin/login";

            res = await fetch(url, {
                method: "POST",
                credentials: "include",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    email: email,
                    password: password
                })
            });

            if (!res.ok) {
                if (res.status === 401) {
                    return false;
                } else {
                    throw new HttpError("HTTP error. Fetch return " + res.status + ".")
                }
            }
        } catch(error) {
            throw error;
        }

        return res;
    }
}
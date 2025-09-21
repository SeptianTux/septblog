'use strict';

import { ChangePasswordModel } from "../Model/ChangePasswordModel.js";
import { HttpError } from "../Errors/HttpError.js";
import { User } from "../Utils/User.js";

export class ChangePasswordViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new ChangePasswordModel(this.#config);
    }

    async checkCredentials() {
        let credentials = false;

        try {
            credentials = await User.checkCredentials();
        } catch(error) {
            throw error;
        }

        return credentials;
    }

    async putPasswordToBackend(oldPassword, newPassword, newPasswordRepeat) {
        let res = null;

        try {
            res = await this.model.putPasswordToBackend(oldPassword, newPassword, newPasswordRepeat);
        } catch(error) {
            throw error
        }

        if (!res.ok) {
            if (res.status === 401) {
                throw new UnauthorizedError("Unauthorized.");
            } else if(res.status === 400) {
                const resJson = await res.json();
                return resJson;
            } else {
                throw new HttpError("HTTP error. " + res.status + ".");
            }
        }

        const resJson = await res.json();

        return resJson;
    }
}
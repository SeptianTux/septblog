'use strict';

import { HttpError } from "../Errors/HttpError.js";
import { ChangeEmailAddressModel } from "../Model/ChangeEmailAddressModel.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { User } from "../Utils/User.js";

export class ChangeEmailAddressViewModel {
    model = null;
    #config = null;

    constructor(config) {
        this.model = new ChangeEmailAddressModel(config);
        this.#config = config;
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

    async changeEmailAddress(newEmailAddress) {
        let res = null;

        try {
            res = await this.model.putEmailToBackend(newEmailAddress);
        } catch(error) {
            throw error;
        }

        if (!res.ok) {
            if (res.status === 401) {
                throw new UnauthorizedError("Unauthorized.");
            } else {
                throw new HttpError("HTTP error. " + res.status + ".");
            }
        }

        const resJson = await res.json();

        return resJson;
    }
}
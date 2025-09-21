'use strict';

import { BadRequestError } from "../Errors/BadRequestError.js";
import { HttpError } from "../Errors/HttpError.js";
import { SettingsModel } from "../Model/SettingsModel.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { InternalServerError } from "../Errors/InternalServerError.js";
import { ForbiddenError } from "../Errors/ForbiddenError.js";

export class SettingsViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new SettingsModel(this.#config);
    }

    async getSettingsDataFromBackend() {
        let res = null;

        try {
            res = await this.model.getSettingsDataFromBackend();
        } catch(error) {
            throw error;
        }

        if (res.status === 200) {
            const resJson = await res.json();

            return resJson;
        } else if (res.status === 401) {
            throw new UnauthorizedError("Unauthorized.");
        } else if (res.status === 403) {
            throw new ForbiddenError("Forbidden.");
        }
        else if (res.status === 500) {
            const resJson = await res.json();
            throw new InternalServerError(resJson.error_message)
        } else {
            throw new HttpError("HTTP error. " + res.status + ".");
        }
    }

    async putSettingsDataToBackend(
        siteTitle,
        tagline,
        signupPage
    ) {
        let res = null;

        try {
            res = await this.model.putSettingsDataToBackend(siteTitle, tagline, signupPage);
        } catch(error) {
            throw error;
        }

        switch (res.status) {
            case 200 : {
                const resJson = await res.json();
                return resJson;
                break;
            }
            case 400 : {
                const resJson = await res.json();
                throw new BadRequestError("Bad request", resJson);
                break;
            }
            case 401 :
                throw new UnauthorizedError("Unauthorized.");
                break;
            case 403 :
                throw new ForbiddenError("Forbidden.");
                break;
            case 500 : {
                const resJson = await res.json();
                throw new InternalServerError(resJson.error_message);
                break;
            }
            default :
                throw new HttpError("HTTP error. " + res.status + ".");
                break;
        }
    }
}
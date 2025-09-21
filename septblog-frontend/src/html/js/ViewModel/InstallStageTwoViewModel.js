'use strict';

import { BadRequestError } from "../../admin/js/Errors/BadRequestError.js";
import { ConflictError } from "../../admin/js/Errors/ConflictError.js";
import { HttpError } from "../../admin/js/Errors/HttpError.js";
import { InternalServerError } from "../../admin/js/Errors/InternalServerError.js";
import { InstallStageTwoModel } from "../Model/InstallStageTwoModel.js";

export class InstallStageTwoViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new InstallStageTwoModel(this.#config);
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
            res = await this.model.putDataToBackend(
                siteTitle,
                tagline,
                firstName,
                lastName,
                username,
                email,
                password1,
                password2
            );
        } catch(error) {
            throw error;
        }

        if (res.status === 200) {
            const resJson = await res.json();

            return resJson;
        } else if (res.status === 500) {
            const resJson = await res.json();

            throw new InternalServerError(resJson.error_message);
        } else if (res.status === 400) {
            const resJson = await res.json();

            throw new BadRequestError("Bad request", resJson);
        } else if (res.status === 409) {
            const resJson = await res.json();

            throw new ConflictError("Already installed.", resJson);
        } else {
            throw new HttpError("HTTP error. " + res.status + ".");
        }
    }

    async alreadyInstalled() {
        let res = null;

        try {
            res = await this.model.alreadyInstalled();
        } catch(error) {
            throw error;
        }

        if (res.status === 200) {
            const resJson = await res.json();

            return resJson;
        } else if (res.status === 500) {
            const resJson = await res.json();
            
            throw new InternalServerError(resJson.error_message);
        } else {
            throw new HttpError("HTTP error. " + res.status + ".");
        }
    }
}
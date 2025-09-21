'use strict';

import { HttpError } from "../Errors/HttpError.js";
import { ProfileModel } from "../Model/ProfileModel.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { BadRequestError } from "../Errors/BadRequestError.js";

export class ProfileViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new ProfileModel(this.#config);
    }

    async getProfileData() {
        let res = null;

        try {
            res = await this.model.getProfileData();
        } catch(error) {
            throw error;
        }

        if (res.ok) {
            const resJson = await res.json();

            return resJson;
        } else {
            if (res.status === 401) {
                throw new UnauthorizedError("Unauthorized.");
            } else {
                throw new HttpError("HTTP error. " + res.status + ".");
            }
        }
    }

    async putProfileData(
        avatar,
        firstName,
        lastName,
        about
    ) {
        let res = null;

        try {
            res = await this.model.putProfileData(
                avatar,
                firstName,
                lastName,
                about
            );
        } catch(error) {
            throw error;
        }

        if (res.ok) {
            const resJson = res.json();

            return resJson;
        } else {
            if (res.status === 401) {
                throw new UnauthorizedError("Unauthorized.");
            } else if (res.status === 400) {
                const resJson = res.json();
                throw new BadRequestError("Bad request.", resJson);
            } else {
                throw new HttpError("HTTP error. " + res.status + ".");
            }
        }
    }

    async imageUpload(xhr) {
        let res = null;

        try {
            res = this.model.imageUpload(xhr);
        } catch(error) {
            throw error;
        }
    }
}
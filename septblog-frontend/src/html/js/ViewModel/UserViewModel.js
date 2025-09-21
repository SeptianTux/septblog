'use strict';

import { UserModel } from "../Model/UserModel.js";
import { HttpError } from "../../admin/js/Errors/HttpError.js";
import { NotFoundError } from "../../admin/js/Errors/NotFoundError.js";

export class UserViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new UserModel(this.#config);
    }

    async getProfileDataFromBackend(username) {
        let res = null;

        try {
            res = await this.model.getProfileDataFromBackend(username)
        } catch(error) {
            console.log(error.message);
            throw error;
        }

        if (res.status === 200) {
            const resJson = await res.json();

            return resJson;
        } else if (res.status === 500) {
            throw new HttpError("HTTP error. " + res.status + ".");
        } else if (res.status === 404) {
            throw new NotFoundError("User not found.");
        } else {
            throw new HttpError("HTTP error. " + res.status + ".");
        }
    } 
}
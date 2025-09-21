'use strict';

import { BadRequestError } from "../Errors/BadRequestError.js";
import { ConflictError } from "../Errors/ConflictError.js";
import { HttpError } from "../Errors/HttpError.js";
import { SignupModel } from "../Model/SignupModel.js";
import { User } from "../Utils/User.js";

export class SignupViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new SignupModel(this.#config);
    }

    async checkCredentials() {
        let check = null;

        try {
            check = await User.checkCredentials();
        } catch(error) {
            throw error;
        }

        return check;
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
            res = await this.model.signup(
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

        let resJson = null;

        switch(res.status) {
            case 201 :
                resJson = await res.json();
                return resJson;
                break;
            case 409 :
                resJson = await res.json();
                throw new ConflictError("Conflict.", resJson);
                break;
            case 400 : {
                resJson = await res.json();
                throw new BadRequestError("Bad request.", resJson);
                break;
            }
            default :
                throw new HttpError("HTTP error. " + res.status + ".");
                break;
        }
    }
}
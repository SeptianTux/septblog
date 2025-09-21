'use strict';

import { ForbiddenError } from "../Errors/ForbiddenError.js";
import { HttpError } from "../Errors/HttpError.js";
import { InternalServerError } from "../Errors/InternalServerError.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { UsersModel } from "../Model/UsersModel.js";

export class UsersViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new UsersModel(this.#config);
    }

    async getUsersDataFromBackend(page) {
        let res = null;

        try {
            res = await this.model.getUsersDataFromBackend(page);
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
        } else if (res.status === 500) {
            throw new HttpError("HTTP error. " + res.status + ".");
        } else {
            throw new HttpError("HTTP error. " + res.status + ".");
        }
    }

    async activateUser(userId) {
        let res = null;

        try {
            res = await this.model.activateUser(userId);
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
        } else if (res.status === 500) {
            const resJson = res.json();

            throw new InternalServerError(resJson.error_message);
        } else  {
            throw new HttpError("HTTP error. " + res.status + ".");
        }
    }

    async suspendUser(userId) {
        let res = null;

        try {
            res = await this.model.suspendUser(userId);
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
        } else if (res.status === 500) {
            const resJson = res.json();

            throw new InternalServerError(resJson.error_message);
        } else  {
            throw new HttpError("HTTP error. " + res.status + ".");
        }
    }

    async deleteUser(userId) {
        let res = null;

        try {
            res = await this.model.deleteUser(userId);
        } catch(error) {
            throw error;
        }

        if (res.status === 200) {
            const resJson = await res.json();
            return resJson;
        } else if (res.status === 401) {
            throw new UnauthorizedError("Unauthorized.");
        } else  {
            throw new HttpError("HTTP error. " + res.status + ".");
        }
    }

    async getUserLevel() {
        let res = null;

        try {
            res = await this.model.getUserLevel();
        } catch(error) {
            throw error;
        }

        if (res.status === 200) {
            const resJson = await res.json();

            return resJson;
        } else if (res.status === 401) {
            throw new UnauthorizedError("Unauthorized.");
        } else if (res.status === 500) {
            const resJson = await res.json();

            throw new HttpError("HTTP error. " + resJson.error_message + ".");
        } else {
            throw new HttpError("HTTP error. " + res.status + ".");
        }
    }
}
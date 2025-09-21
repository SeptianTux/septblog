'use strict';

import { LogoutModel } from "../Model/LogoutModel.js";

export class LogoutViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new LogoutModel(this.#config);
    }

    logout() {
        try {
            localStorage.clear();
        } catch(error) {
            throw error;
        }
    }
}
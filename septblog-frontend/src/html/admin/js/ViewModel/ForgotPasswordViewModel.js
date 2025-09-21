'use strict';

import { ForgotPasswordModel } from "../Model/ForgotPasswordModel.js";

export class ForgotPasswordViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new ForgotPasswordModel(this.#config);
    }
}
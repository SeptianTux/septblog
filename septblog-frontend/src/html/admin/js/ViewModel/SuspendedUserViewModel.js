'use strict';

import { SuspendedUserModel } from "../Model/SuspendedUserModel.js";

export class SuspendedUserViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new SuspendedUserModel(this.#config);
    }
}
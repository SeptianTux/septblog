'use strict';

import { NotFoundModel } from "../Model/404Model.js";

export class NotFoundViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new NotFoundModel(this.#config);
    }
}
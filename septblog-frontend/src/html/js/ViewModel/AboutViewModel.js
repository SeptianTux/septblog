'use strict';

import { AboutModel } from "../Model/AboutModel.js";

export class AboutViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new AboutModel(this.#config);
    }
}
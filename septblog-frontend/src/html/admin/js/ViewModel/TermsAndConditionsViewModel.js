'use strict';

import { TermsAndConditionsModel } from "../Model/TermsAndConditionsModel.js";

export class TermsAndConditionsViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new TermsAndConditionsModel(this.#config);
    }
}
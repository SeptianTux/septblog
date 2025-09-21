'use strict';

import { PrivacyPolicyModel } from "../Model/PrivacyPolicyModel.js";

export class PrivacyPolicyViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new PrivacyPolicyModel(this.#config);
    }
}
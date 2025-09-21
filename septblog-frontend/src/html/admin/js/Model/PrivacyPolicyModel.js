'use strict';

export class PrivacyPolicyModel {
    #config = null;

    constructor(config) {
        this.#config = config;

        console.log("yay from model.");
    }
}
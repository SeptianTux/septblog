'use strict';

export class AboutModel {
    #config = null;

    constructor(config) {
        this.#config = config;

        console.log("Hello from model.");
    }
}
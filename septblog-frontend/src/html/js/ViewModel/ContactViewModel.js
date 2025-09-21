'use strict';

import { ContactModel } from "../Model/ContactModel.js";

export class ContactViewModel {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.model = new ContactModel(this.#config);
    }
}
'use strict';

import { NotFoundViewModel } from "../ViewModel/404ViewModel.js";

export class NotFoundModel {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config;
    }
}
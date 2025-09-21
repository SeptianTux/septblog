'use strict';

import { Page } from "../../admin/js/Utils/Page.js";
import { NotFoundViewModel } from "../ViewModel/404ViewModel.js";

export class NotFoundView {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new NotFoundViewModel(this.#config);

        this.#setSiteTitle();
    }

    async #setSiteTitle() {
        await Page.setSiteTitle();
    }
}
'use strict';

import { Page } from "../Utils/Page.js";
import { SuspendedUserViewModel } from "../ViewModel/SuspendedUserViewModel.js";

export class SuspendedUserView {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new SuspendedUserViewModel(this.#config);

        this.#setPageTitle();
        this.#setCopyright();
    }

    async #setCopyright() {
        await Page.setCopyright();
    }

    async #setPageTitle() {
        await Page.setSiteTitleAdmin();
    }
}
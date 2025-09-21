'use strict';

import { Page } from "../Utils/Page.js";
import { TermsAndConditionsViewModel } from "../ViewModel/TermsAndConditionsViewModel.js";

export class TermsAndConditionsView {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new TermsAndConditionsViewModel(this.#config);

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
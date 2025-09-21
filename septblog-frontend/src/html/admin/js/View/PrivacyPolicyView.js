'use strict';

import { Page } from "../Utils/Page.js";
import { PrivacyPolicyViewModel } from "../ViewModel/PrivacyPolicyViewModel.js";

export class PrivacyPolicyView {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new PrivacyPolicyViewModel(this.#config);

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
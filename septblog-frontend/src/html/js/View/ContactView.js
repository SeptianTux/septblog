'use strict';

import { Page } from "../Utils/Page.js";
import { ContactViewModel } from "../ViewModel/ContactViewModel.js";

export class ContactView {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new ContactViewModel(this.#config);

        this.#setSiteTitle();
        this.#setNavbarBrand();
        this.#setCopyright();
    }

    async #setSiteTitle() {
        await Page.setSiteTitle();
    }

    async #setNavbarBrand() {
        await Page.setNavbarBrand();
    }

    async #setCopyright() {
        await Page.setCopyright();
    }
}
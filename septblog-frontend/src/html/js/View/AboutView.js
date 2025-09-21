'use strict';

import { Page } from "../../admin/js/Utils/Page.js";
import { AboutViewModel } from "../ViewModel/AboutViewModel.js";

export class AboutView {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new AboutViewModel(this.#config);

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
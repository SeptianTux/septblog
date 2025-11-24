'use strict';

import { Page } from "../Utils/Page.js";
import { User } from "../Utils/User.js";

export class SecurityView {
    #config = null;

    constructor(config) {
        this.#config = config;

        this.#checkCredentials();

        this.#setLoggedInAs();
        this.#setPageTitle();
        this.#setNavbarBrand();
        this.#setCopyright();

        this.#administratorsNavMenu();
    }

    async #checkCredentials() {
        const checkCredentials = await User.checkCredentials();

        if (!checkCredentials) {
            this.redirectToLoginPage();
        } else {
            document.getElementById('body').style.display = 'block';
        }
    }

    async #administratorsNavMenu() {
        await User.administratorsNavMenu();
    }

    async #setCopyright() {
        await Page.setCopyright();
    }

    async #setPageTitle() {
        await Page.setSiteTitleAdmin();
    }

    async #setNavbarBrand() {
        await Page.setNavbarBrand();
    }

    async #setLoggedInAs() {
        await Page.setLoggedInAs();
    }

    redirectToLoginPage() {
        const path = "/admin/profile";
        const redirectTo = encodeURIComponent(path);

        window.location.href = "/admin/login?redirect-to=" + redirectTo;
    }
}
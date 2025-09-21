'use strict';

import { Page } from "../Utils/Page.js";
import { LogoutViewModel } from "../ViewModel/LogoutViewModel.js";

export class LogoutView {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new LogoutViewModel(this.#config);

        this.#setPageTitle();
        this.#setCopyright();

        this.#logout();
    }

    async #setCopyright() {
        await Page.setCopyright();
    }

    async #setPageTitle() {
        await Page.setSiteTitleAdmin();
    }

    async #logout() {
        try {
            this.viewModel.logout();
        } catch(e) {
            console.log(e);
        }

        this.alertSuccess("Logout success, please wait...");

        setTimeout(function() {
            window.location.href = "/admin/login";
        }, 2000);

        return false;
    }

    alertDanger(message) {
        const alert = '<div id="alert" class="alert alert-danger alert-dismissible fade show" role="alert">' +
                            message +
                            '<button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>' +
                        '</div>';
        
        document.getElementById("alertContainer").insertAdjacentHTML("beforeend", alert);
    }

    alertSuccess(message) {
        const alert = '<div id="alert" class="alert alert-success alert-dismissible fade show" role="alert">' +
                            message +
                            '<button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>' +
                        '</div>';
        
        document.getElementById("alertContainer").insertAdjacentHTML("beforeend", alert);
    }
}
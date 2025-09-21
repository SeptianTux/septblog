'use strict';

import { Page } from "../Utils/Page.js";
import { User } from "../Utils/User.js";
import { ForgotPasswordViewModel } from "../ViewModel/ForgotPasswordViewModel.js";

export class ForgotPasswordView {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new ForgotPasswordViewModel(this.#config);

        this.alertDanger("This feature is not available now, please contact administrator if you forgot your password.");
        this.disableButton();
        this.disableInput();
        this.#setCopyright();
        this.#setPageTitle();
        this.#setNavbarBrand();

        this.#administratorsNavMenu();
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

    submit() {
        return false;
    }

    disableButton() {
        return document.getElementById("submitButton").disabled = true;
    }

    disableInput() {
        return document.getElementById("email").disabled = true;
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

    alertClose() {
        const alertElement = document.getElementById('alert');

        if (alertElement !== null) {
            const alertInstance = bootstrap.Alert.getOrCreateInstance(alertElement);

            alertInstance.close();
        }
    }
}
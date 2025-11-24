'use strict';

import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { ChangeEmailAddressViewModel } from "../ViewModel/ChangeEmailAddressViewModel.js";
import { Page } from "../Utils/Page.js";
import { User } from "../Utils/User.js";

export class ChangeEmailAddressView {
    viewModel = null;
    #config = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new ChangeEmailAddressViewModel(this.#config);

        this.#checkCredentials();
        this.#formEnterKeyListener();

        this.#setLoggedInAs();
        this.#setPageTitle();
        this.#setNavbarBrand();
        this.#setCopyright();

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

    async #setLoggedInAs() {
        await Page.setLoggedInAs();
    }

    async #checkCredentials() {
        const checkCredentials = await User.checkCredentials();

        if (!checkCredentials) {
            this.#redirectToLoginPage();
        } else {
            document.getElementById('body').style.display = 'block';
        }
    }

    #validation() {
        let isValid = true;

        if (document.getElementById("newEmailAddress").value === "") {
            isValid = false;
            this.newEmailAddressInputInvalid("Please fill the New Email Address form.");
        } else {
            this.newEmailAddressInputValid();
        }

        return isValid;
    }

    #formEnterKeyListener() {
        document.getElementById("form").addEventListener("keydown", function(event) {
            if (event.key === "Enter") {
                main.view.submit();
            }
        });
    }

    async changeEmailAddress() {
        const newEmailAddress = document.getElementById("newEmailAddress").value;
        let res = null;

        try {
            res = await this.viewModel.changeEmailAddress(newEmailAddress);
        } catch(error) {
            if (error instanceof UnauthorizedError) {
                this.#redirectToLoginPage();
            } else {
                this.alertClose();
                this.alertDanger(error.message);
            }
        }

        if (res.response === true) {
            localStorage.setItem("access_token", res.access_token);

            this.alertClose();
            this.alertSuccess("Email changed.");
        }
    }

    async submit() {
        const validation = this.#validation();
        if(validation === true) {
            this.buttonLoading();
            await this.changeEmailAddress();

            setTimeout(function() {
                main.view.buttonNormal();
            }, 1000);
        }
    }

    newEmailAddressInputInvalid(message) {
        if (message != null) {
            document.getElementById("newEmailAddressInputInvalidFeedback").innerText = message;
        }
        
        document.getElementById("newEmailAddress").classList.add('is-invalid');
    }

    newEmailAddressInputValid() {
        document.getElementById("newEmailAddress").classList.remove('is-invalid');
    }

    #buttonLoading(buttonId) {
        let btn = document.getElementById(buttonId);
        const spinner = '<span class="spinner-border spinner-border-sm" aria-hidden="true"></span>' +
                        '<span role="status">Loading...</span>';

        if (btn === null) {
            return false;
        }

        btn.disabled = true;
        btn.replaceChildren();
        btn.insertAdjacentHTML("beforeend", spinner);

        return true;
    }

    #buttonNormal(buttonId, buttonName) {
        let btn = document.getElementById(buttonId);

        if (btn === null) {
            return false;
        }

        btn.replaceChildren();
        btn.textContent = buttonName;
        btn.disabled = false;

        return true;
    }

    buttonLoading() {
        return this.#buttonLoading("submit");
    }

    buttonNormal() {
        return this.#buttonNormal("submit", "Submit");
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

        if (alertElement) {
            const alertInstance = bootstrap.Alert.getOrCreateInstance(alertElement);
            alertInstance.close();
        }
    }

    #redirectToLoginPage() {
        const path = "/admin/security/change-email";
        const redirectTo = encodeURIComponent(path);

        window.location.href = "/admin/login?redirect-to=" + redirectTo;
    }
}
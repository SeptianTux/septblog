'use strict';

import { HttpError } from "../Errors/HttpError.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { ChangePasswordViewModel } from "../ViewModel/ChangePasswordViewModel.js";
import { Page } from "../Utils/Page.js";
import { User } from "../Utils/User.js";

export class ChangePasswordView {
    #config = null;
    model = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new ChangePasswordViewModel(this.#config);

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
        let credentials = null;
        
        try {
            credentials = await this.viewModel.checkCredentials();
        } catch(error) {
            this.alertDanger("Problem in connecting to the server.");
        }

        if (credentials === false) {
            this.#redirectToLoginPage();
        }
    }

    #validation() {
        let isValid = true;

        if (document.getElementById("oldPassword").value === "") {
            this.isValid = false;
            this.oldPasswordInputInvalid("Please fill the Old Password form.");
        } else {
            this.oldPasswordInputValid();
        }

        if (document.getElementById("newPassword1").value === "") {
            this.isValid = false;
            this.newPassword1InputInvalid("Please fill the New Password form.");
        } else {
            this.newPassword1InputValid();
        }

        if (document.getElementById("newPassword2").value === "") {
            this.isValid = false;
            this.newPassword2InputInvalid("Please fill the New Password form.");
        } else {
            this.newPassword2InputValid();
        }

        if ((document.getElementById("newPassword1").value !== "" && document.getElementById("newPassword2").value !== "") && (document.getElementById("newPassword1").value !== document.getElementById("newPassword2").value)) {
            isValid = false;
            this.newPassword1InputInvalid("Password doesn't match.");
            this.newPassword2InputInvalid("Password doesn't match.");
        }

        return isValid;
    }

    #formEnterKeyListener() {
        document.getElementById("form").addEventListener("keydown", function(event) {
            if (event.key === "Enter") {
                custom.importedClass.submit();
            }
        });
    }

    async putPasswordToBackend() {
        let res = null;
        const oldPassword = this.getOldPasswordValue();
        const newPassword1 = this.getNewPassword1Value();
        const newPassword2 = this.getNewPassword2Value();

        try {
            res = await this.viewModel.putPasswordToBackend(oldPassword, newPassword1, newPassword2);
        } catch(error) {
            if (error instanceof UnauthorizedError) {
                this.#redirectToLoginPage();
            } else if (error instanceof HttpError) {
                this.alertDanger(error.message);
            } else {
                this.alertDanger(error.message);
            }
        }

        if (res.error) {
            if (res.error_code === 679) {
                this.oldPasswordInputInvalid("Invalid old password.");
            } else if (res.error_code === 45) {
                this.newPassword1InputInvalid("Password doesn't match.");
                this.newPassword2InputInvalid("Password doesn't match.");
            }
        } else if (res.response === true) {
            this.alertSuccess("Password changed.");
        }
    }

    async submit() {
        const valid = this.#validation();

        if (valid) {
            this.buttonLoading();
            await this.putPasswordToBackend();
            this.buttonNormal();
        }
    }

    getOldPasswordValue() {
        return document.getElementById("oldPassword").value;
    }

    getNewPassword1Value() {
        return document.getElementById("newPassword1").value;
    }

    getNewPassword2Value() {
        return document.getElementById("newPassword2").value;
    }

    oldPasswordInputInvalid(message) {
        if (message != null) {
            document.getElementById("oldPasswordInputInvalidFeedback").innerText = message;
        }
        
        document.getElementById("oldPassword").classList.add('is-invalid');
    }

    newPassword1InputInvalid(message) {
        if (message != null) {
            document.getElementById("mewPassword1InputInvalidFeedback").innerText = message;
        }
        
        document.getElementById("newPassword1").classList.add('is-invalid');
    }

    newPassword2InputInvalid(message) {
        if (message != null) {
            document.getElementById("mewPassword2InputInvalidFeedback").innerText = message;
        }
        
        document.getElementById("newPassword2").classList.add('is-invalid');
    }

    oldPasswordInputValid() {
        document.getElementById("oldPassword").classList.remove('is-invalid');
    }

    newPassword1InputValid() {
        document.getElementById("newPassword1").classList.remove('is-invalid');
    }

    newPassword2InputValid() {
        document.getElementById("newPassword2").classList.remove('is-invalid');
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

        if (alertElement !== null) {
            const alertInstance = bootstrap.Alert.getOrCreateInstance(alertElement);

            alertInstance.close();
        }
    }

    #redirectToLoginPage() {
        const path = "/admin/security/change-password";
        const redirectTo = encodeURIComponent(path);

        window.location.href = "/admin/login?redirect-to=" + redirectTo;
    }
}
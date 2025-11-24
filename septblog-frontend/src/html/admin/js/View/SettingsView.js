'use strict';

import { BadRequestError } from "../Errors/BadRequestError.js";
import { HttpError } from "../Errors/HttpError.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { SettingsViewModel } from "../ViewModel/SettingsViewModel.js";
import { Page } from "../Utils/Page.js";
import { User } from "../Utils/User.js";
import { ForbiddenError } from "../Errors/ForbiddenError.js";

export class SettingsView {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new SettingsViewModel(this.#config);

        this.#checkCredentials();

        this.#checkPrivilage();

        this.getSettingsDataFromBackend();
        this.#formEnterKeyListener();

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

    async #checkPrivilage() {
        let userLevel = null;
        
        try {
            userLevel = await User.getUserLevel();
        } catch(error) {
            if (error instanceof UnauthorizedError) {
                this.redirectToLoginPage();
            } else {
                this.alertDanger(error.message);
            }
        }

        if (userLevel === 'user') {
            this.#redirectToDashboard();
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

    #formEnterKeyListener() {
        document.getElementById("form").addEventListener("keydown", function(event) {
            if (event.key === "Enter") {
                main.view.submit();
            }
        });
    }

    #buttonLoading(buttonId) {
        const btn = document.getElementById(buttonId);
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
        const btn = document.getElementById(buttonId);

        if (btn === null) {
            return false;
        }

        btn.replaceChildren();
        btn.textContent = buttonName;
        btn.disabled = false;

        return true;
    }

    #validation() {
        let isValid = true;

        if (document.getElementById("siteTitle").value === "") {
            isValid = false;
            this.siteTitleInputInvalid();
        } else {
            this.siteTitleInputValid();
        }

        if (document.getElementById("tagline").value === "") {
            isValid = false;
            this.taglineInputInvalid();
        } else {
            this.taglineInputValid();
        }

        return isValid;
    }

    async getSettingsDataFromBackend() {
        let res = null;

        try {
            res = await this.viewModel.getSettingsDataFromBackend();
        } catch(error) {
            if (error instanceof UnauthorizedError) {
                this.redirectToLoginPage();
            } else if (error instanceof HttpError) {
                this.alertDanger(error.message);
            } else if (error instanceof TypeError) {
                this.alertDanger("Failed to connect to the network.");
            } else if (error instanceof ForbiddenError) {
                this.#redirectToDashboard();
            } else {
                this.alertDanger(error.message);
            }
        }

        if (res.response === true) {
            this.setSiteTitleValue(res.data.site_title);
            this.setTaglineValue(res.data.site_tagline);
            this.setSignupPageCheckboxValue(res.data.enable_signup_page);
        } else {
            this.alertDanger(res.error_message);
        }
    }

    async putSettingsDataToBackend(
        siteTitle,
        tagline,
        signupPage
    ) {
        let res = null;

        try {
            res = await this.viewModel.putSettingsDataToBackend(siteTitle, tagline, signupPage);
        } catch(error) {
            if (error instanceof UnauthorizedError) {
                this.redirectToLoginPage();
            } else if (error instanceof TypeError) {
                this.alertDanger("Failed to connect to the network.");
            } else if (error instanceof BadRequestError) {
                if (error.response.error_code === 1) {
                    this.siteTitleInputInvalid();
                } else if (error.response.error_code === 2) {
                    this.taglineInputInvalid();
                }
            } else if (error instanceof ForbiddenError) {
                this.#redirectToDashboard();
            } else if (error instanceof HttpError) {
                this.alertDanger(error.message);
            } else {
                this.alertDanger(error.message);
            }
        }

        if (res.response === true) {
            this.alertSuccess("Data saved.");
        } else {
            this.alertDanger("Failed to save data.");
        }
    }

    async submit() {
        const validation = this.#validation();

        this.#buttonLoading();

        if (validation) {
            const siteTitle = this.getSiteTitleValue();
            const tagline = this.getTaglineValue();
            const signupPage = this.getSignupPageValue();

            await this.putSettingsDataToBackend(siteTitle, tagline, signupPage);
        }

        this.#buttonNormal();
    }

    getSiteTitleValue() {
        const title = document.getElementById("siteTitle").value;

        return title;
    }

    getTaglineValue() {
        const tagline = document.getElementById("tagline").value;

        return tagline;
    }

    getSignupPageValue() {
        const checkbox = document.getElementById("signupPage");
        
        return checkbox.checked;
    }

    setSiteTitleValue(value) {
        const siteTitle = document.getElementById("siteTitle");

        siteTitle.value = value;
    }

    setTaglineValue(value) {
        const tagline = document.getElementById("tagline");

        tagline.value = value;
    }

    // It takes boolean value for argument
    setSignupPageCheckboxValue(status) {
        const checkbox = document.getElementById("signupPage");

        checkbox.checked = status
    }

    alertDanger(message) {
        let alert = '<div id="alert" class="alert alert-danger alert-dismissible fade show" role="alert">' +
                        message +
                        '<button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>' +
                    '</div>';
        
        document.getElementById("alertSettings").insertAdjacentHTML("beforeend", alert);
    }

    alertSuccess(message) {
        let alert = '<div id="alert" class="alert alert-success alert-dismissible fade show" role="alert">' +
                        message +
                        '<button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>' +
                    '</div>';
        
        document.getElementById("alertSettings").insertAdjacentHTML("beforeend", alert);
    }

    alertClose() {
        const alertElement = document.getElementById('alert');
        const alertInstance = bootstrap.Alert.getOrCreateInstance(alertElement);

        alertInstance.close();
    }

    saveButtonLoading() {
        return this.#buttonLoading("saveButton");
    }

    saveButtonNormal() {
        return this.#buttonNormal("saveButton", "Save");
    }

    siteTitleInputInvalid(message) {
        if (message != null) {
            document.getElementById("siteTitleInputInvalidFeedback").innerText = message;
        }
        
        document.getElementById("siteTitle").classList.add('is-invalid');
    }

    taglineInputInvalid(message) {
        if (message != null) {
            document.getElementById("taglineInputInvalidFeedback").innerText = message;
        }
        
        document.getElementById("tagline").classList.add('is-invalid');
    }

    siteTitleInputValid() {
        document.getElementById("siteTitle").classList.remove('is-invalid');
    }

    taglineInputValid() {
        document.getElementById("tagline").classList.remove('is-invalid');
    }

    redirectToLoginPage() {
        const path = "/admin/settings";
        const redirectTo = encodeURIComponent(path);

        window.location.href = "/admin/login?redirect-to=" + redirectTo;
    }

    #redirectToDashboard() {
        window.location.href = "/admin";
    }
}
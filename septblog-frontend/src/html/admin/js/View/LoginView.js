'use strict';

import { Page } from "../Utils/Page.js";
import { LoginViewModel } from "../ViewModel/LoginViewModel.js";

export class LoginView {
    viewModel = null;
    config = null;

    constructor(config) {
        this.config = config;
        this.viewModel = new LoginViewModel(this.config);
        this.checkCredentials();
        this.#formEnterKeyListener();
        this.#setPageTitle();
        this.#setCopyright();
    }

    async #setCopyright() {
        await Page.setCopyright();
    }

    async #setPageTitle() {
        await Page.setSiteTitleAdmin();
    }

    async checkCredentials() {
        let check = null;
        
        try {
            check = await this.viewModel.checkCredentials();
        } catch(error) {
            if (error instanceof TypeError) {
                this.alertDanger("Problem in connecting to the server.");
            }
        }
        
        if (check) {
            setTimeout(function() {
                main.view.alertSuccess("You are already logged in. Please wait...");
            }, 1000);

            setTimeout(function() {
                const redirectTo = main.view.getRedirectUrl();

                if (redirectTo === null) {
                    window.location.href = "/admin";
                } else {
                    window.location.href = redirectTo;
                }
            }, 2000);
        }
    }

    #disableForm() {
        const emailForm     = document.getElementById("email");
        const passwordForm  = document.getElementById("password");

        if (emailForm) {
            emailForm.disabled = true;
        }
        if (passwordForm) {
            passwordForm.disabled = true;
        }
    }

    #enableForm() {
        const emailForm     = document.getElementById("email");
        const passwordForm  = document.getElementById("password");

        if (emailForm) {
            emailForm.disabled      = false;
        }
        if (passwordForm) {
            passwordForm.disabled   = false;
        }
    }

    getRedirectUrl() {
        const queryString   = window.location.search;
        const urlParams     = new URLSearchParams(queryString);
        const redirectTo    = urlParams.get('redirect-to');

        return redirectTo;
    }

    async submit() {
        const validation = this.#validation();

        this.#disableForm();
        this.buttonLoading();

        if (validation) {
            let login = null;

            try {
                login = await this.viewModel.login(this.getEmailValue(), this.getPasswordValue());
            } catch(error) {
                if (error instanceof TypeError) {
                    this.alertClose();
                    this.alertDanger("Problem in connecting to the server.");
                    this.buttonNormal();
                }
            }

            if (login.login) {
                localStorage.setItem("access_token", login.data.access_token);

                setTimeout(function() {
                    main.view.alertSuccess("Login success. Please wait...");
                }, 1000);

                setTimeout(function() {
                    const redirectTo = main.view.getRedirectUrl();

                    if (redirectTo === null) {
                        window.location.href = "/admin";
                    } else {
                        window.location.href = redirectTo;
                    }
                }, 2000);
            } else if (login === false) {
                main.view.alertDanger("Invalid email and password combination.");
                this.buttonNormal();
                this.#enableForm();
            }
        }
    }

    #validation() {
        let isValid = true;

        if (document.getElementById("email").value === "") {
            isValid = false;
            this.emailInputInvalid();
        } else {
            this.emailInputValid();
        }

        if (document.getElementById("password").value === "") {
            isValid = false;
            this.passwordInputInvalid();
        } else {
            this.passwordInputValid();
        }

        return isValid;
    }

    getEmailValue() {
        return document.getElementById("email").value;
    }

    getPasswordValue() {
        return document.getElementById("password").value;
    }

    #formEnterKeyListener() {
        document.getElementById("form").addEventListener("keydown", function(event) {
            if (event.key === "Enter") {
                main.view.submit();
            }
        });
    }

    buttonLoading() {
        const btn = document.getElementById("submitButton");
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

    buttonNormal() {
        const btn = document.getElementById("submitButton");

        if (btn === null) {
            return false;
        }

        btn.replaceChildren();
        btn.textContent = "Login";
        btn.disabled = false;

        return true;
    }

    emailInputInvalid(message) {
        if (message != null) {
            document.getElementById("emailInputInvalidFeedback").innerText = message;
        }
        
        document.getElementById("email").classList.add('is-invalid');
    }

    passwordInputInvalid(message) {
        if (message != null) {
            document.getElementById("passwordInputInvalidFeedback").innerText = message;
        }
        
        document.getElementById("password").classList.add('is-invalid');
    }

    emailInputValid() {
        document.getElementById("email").classList.remove('is-invalid');
    }

    passwordInputValid() {
        document.getElementById("password").classList.remove('is-invalid');
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

    alertCloseIfExist() {
        const alertElement = document.getElementById('alert');

        if (alertElement !== null) {
            const alertInstance = bootstrap.Alert.getOrCreateInstance(alertElement);
            alertInstance.close();
        }
    }
}
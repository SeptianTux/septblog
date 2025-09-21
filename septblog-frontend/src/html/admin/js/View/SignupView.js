'use strict';

import { BadRequestError } from "../Errors/BadRequestError.js";
import { ConflictError } from "../Errors/ConflictError.js";
import { HttpError } from "../Errors/HttpError.js";
import { Page } from "../Utils/Page.js";
import { SignupViewModel } from "../ViewModel/SignupViewModel.js";

export class SignupView {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new SignupViewModel(this.#config);

        this.#checkCredentials();
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

    async #checkCredentials() {
        let check = null;

        try {
            check = await this.viewModel.checkCredentials();
        } catch(error) {
            if (error instanceof TypeError) {
                this.alertDanger("Failed to connect to the network.");
            } else {
                this.alertDanger(error.message);
            }
        }

        if (check === true) {
            this.#buttonLoading();
            this.#inputFormDisable();
            this.alertSuccess("You are already logged in, please wait....")

            setTimeout(function() {
                main.view.redirect("/admin");
            }, 3000);
        }
    }

    #validation() {
        let isValid = true;

        if (document.getElementById("inputFirstName").value === "") {
            isValid = false;
            this.inputFirstNameInvalid("Please fill the First Name form.");
        } else {
            this.inputFirstNameValid();
        }

        if (document.getElementById("inputUsername").value === "") {
            isValid = false;
            this.inputUsernameInvalid("Please choose a username.");
        } else {
            this.inputUsernameValid();
        }

        if (document.getElementById("inputEmail").value === "") {
            isValid = false;
            this.inputEmailInvalid("Please fill an email address.");
        } else {
            this.inputEmailValid();
        }

        if (document.getElementById("inputPassword1").value === "") {
            isValid = false;
            this.inputPassword1Invalid("Please fill a password.");
        } else {
            this.inputPassword1Valid();
        }

        if (document.getElementById("inputPassword2").value === "") {
            isValid = false;
            this.inputPassword2Invalid("Please confirm the password.");
        } else {
            this.inputPassword2Valid();
        }

        if ((document.getElementById("inputPassword1").value !== "" && document.getElementById("inputPassword2").value !== "") && (document.getElementById("inputPassword1").value !== document.getElementById("inputPassword2").value)) {
            isValid = false;
            this.inputPassword1Invalid("Password doesn't match.");
            this.inputPassword2Invalid("Password doesn't match.");
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

    #buttonLoading(buttonId) {
        let btn = document.getElementById(buttonId);
        let spinner = '<span class="spinner-border spinner-border-sm" aria-hidden="true"></span>' +
                        '<span role="status">Loading...</span>';

        if (btn == null) {
            return false;
        }

        btn.disabled = true;
        btn.replaceChildren();
        btn.insertAdjacentHTML("beforeend", spinner);

        return true;
    }

    #buttonNormal(buttonId, buttonName) {
        let btn = document.getElementById(buttonId);

        if (btn == null) {
            return false;
        }

        btn.replaceChildren();
        btn.textContent = buttonName;
        btn.disabled = false;

        return true;
    }

    #inputFormDisable(inputId) {
        const input = document.getElementById(inputId);

        if (input) {
            input.disabled = true;
        }
    }

    #inputFormEnable(inputId) {
        const input = document.getElementById(inputId);

        if (input) {
            input.disabled = false;
        }
    }

    async #signup() {
        let res = null;

        this.createAccountButtonLoading();
        this.createAccountFormDisable();

        try {
            const firstName = this.getFirstName();
            const lastName = this.getLastName();
            const username = this.getUsername();
            const email = this.getEmail();
            const password1 = this.getPassword1();
            const password2 = this.getPassword2();

            res = await this.viewModel.signup(
                firstName,
                lastName,
                username,
                email,
                password1,
                password2
            );
        } catch(error) {
            if (error instanceof ConflictError) {
                if (error.response.error_code === 11) {
                    main.view.inputUsernameInvalid(error.response.error_message);
                } else {
                    main.view.inputUsernameValid();
                }

                if (error.response.error_code === 12) {
                    main.view.inputEmailInvalid(error.response.error_message);
                } else {
                    main.view.inputEmailValid();
                }
            } else if (error instanceof BadRequestError) {
                const res = error.response;

                if (res.error_code === 32) {
                    main.view.inputUsernameInvalid("Only a-z, A-Z, 0-9, and _ characters allowed.");
                } else if (res.error_code === 34) {
                    main.view.inputUsernameInvalid("Username is empty.");
                } else {
                    main.view.inputUsernameValid();
                }

                if (res.error_code === 33) {
                    main.view.inputFirstNameInvalid("First name is empty.");
                } else {
                    main.view.inputFirstNameValid();
                }

                if (res.error_code === 35) {
                    main.view.inputEmailInvalid("Email is empty.");
                } else {
                    main.view.inputEmailValid();
                }

                if (res.error_code === 36) {
                    main.view.inputPassword1Invalid("Password is empty.");
                } else if (res.error_code === 37) {
                    main.view.inputPassword2Invalid("Please confirm password.");
                } else if (res.error_code === 38) {
                    main.view.inputPassword1Invalid("Password doesn't match.");
                    main.view.inputPassword2Invalid("Password doesn't match.");
                } else {
                    main.view.inputPassword1Valid();
                    main.view.inputPassword2Valid();
                }
            } else if (error instanceof HttpError) {
                this.alertDanger("HTTP error. " + error.message + ".");
            } else {
                this.alertDanger(error.message);
            }
        }

        if (res.response === true && res.signup === true) {
            localStorage.setItem("access_token", res.data.access_token);

            setTimeout(function() {
                main.view.alertSuccess("Signup success. Please wait...");
            }, 1000);

            setTimeout(function() {
                window.location.href = "/admin";
            }, 3000);
        } else {
            main.alertDanger("Failed to create a new user.");

            this.createAccountFormEnable();
            this.createAccountButtonNormal();
        }
    }

    submit() {
        const valid = this.#validation();

        if (valid) {
            

            this.#signup();

            
        }
    }

    alertSuccess(message) {
        const alert = '<div id="alert" class="alert alert-success alert-dismissible fade show" role="alert">' +
                            '<div>' + message + '</div>' +
                            '<button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>' +
                        '</div>';

        document.getElementById("signupAlert").insertAdjacentHTML("beforeend", alert);
    }

    alertDanger(message) {
        const alert = '<div id="alert" class="alert alert-danger alert-dismissible fade show" role="alert">' +
                            '<div>' + message + '</div>' +
                            '<button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>' +
                        '</div>';
        
        document.getElementById("signupAlert").insertAdjacentHTML("beforeend", alert);
    }

    alertClose() {
        const alertElement = document.getElementById('alert');
        const alertInstance = bootstrap.Alert.getOrCreateInstance(alertElement); // Get the Bootstrap alert instance

        alertInstance.close();
    }

    alertCloseIfExist() {
        const alertElement = document.getElementById('alert');

        if (alertElement) {
            const alertInstance = bootstrap.Alert.getOrCreateInstance(alertElement); // Get the Bootstrap alert instance
            alertInstance.close();
        }
    }

    inputFirstNameInvalid(message) {
        if (message != null)
            document.getElementById("inputFirstNameInvalidFeedback").innerText = message;

        document.getElementById("inputFirstName").classList.add('is-invalid');
    }

    inputFirstNameValid() {
        document.getElementById("inputFirstName").classList.remove('is-invalid');
    }

    inputLastNameInvalid(message) {
        if (message != null)
            document.getElementById("inputLastNameInvalidFeedback").innerText = message;
        
        document.getElementById("inputLastName").classList.add('is-invalid');
    }

    inputLastNameValid() {
        document.getElementById("inputLastName").classList.remove('is-invalid');
    }

    inputUsernameInvalid(message) {
        if (message != null)
            document.getElementById("inputUsernameInvalidFeedback").innerText = message;

        document.getElementById("inputUsername").classList.add('is-invalid');
    }

    inputUsernameValid() {
        document.getElementById("inputUsername").classList.remove('is-invalid');
    }

    inputEmailInvalid(message) {
        if (message != null)
            document.getElementById("inputEmailInvalidFeedback").innerText = message;

        document.getElementById("inputEmail").classList.add('is-invalid');
    }

    inputEmailValid() {
        document.getElementById("inputEmail").classList.remove('is-invalid');
    }

    inputPassword1Invalid(message) {
        if (message != null)
            document.getElementById("inputPassword1InvalidFeedback").innerText = message;

        document.getElementById("inputPassword1").classList.add('is-invalid');
    }

    inputPassword1Valid() {
        document.getElementById("inputPassword1").classList.remove('is-invalid');
    }

    inputPassword2Invalid(message) {
        if (message != null)
            document.getElementById("inputPassword2InvalidFeedback").innerText = message;

        document.getElementById("inputPassword2").classList.add('is-invalid');
    }

    inputPassword2Valid() {
        document.getElementById("inputPassword2").classList.remove('is-invalid');
    }

    createAccountButtonLoading() {
        return this.#buttonLoading("createAccountButton");
    }

    createAccountButtonNormal() {
        return this.#buttonNormal("createAccountButton", "Create Account");
    }

    createAccountFormDisable() {
        this.#inputFormDisable("inputFirstName");
        this.#inputFormDisable("inputLastName");
        this.#inputFormDisable("inputUsername");
        this.#inputFormDisable("inputEmail");
        this.#inputFormDisable("inputPassword1");
        this.#inputFormDisable("inputPassword2");
    }

    createAccountFormEnable() {
        this.#inputFormEnable("inputFirstName");
        this.#inputFormEnable("inputLastName");
        this.#inputFormEnable("inputUsername");
        this.#inputFormEnable("inputEmail");
        this.#inputFormEnable("inputPassword1");
        this.#inputFormEnable("inputPassword2");
    }

    getFirstName() {
        const firstName = document.getElementById("inputFirstName").value;

        if (firstName) {
            return firstName;
        }
    }

    getLastName() {
        const lastName = document.getElementById("inputLastName").value;

        if (lastName) {
            return lastName;
        }
    }

    getUsername() {
        const username = document.getElementById("inputUsername").value;

        if (username) {
            return username;
        }
    }

    getEmail() {
        const email = document.getElementById("inputEmail").value;

        if (email) {
            return email;
        }
    }

    getPassword1() {
        const password = document.getElementById("inputPassword1").value;

        if (password) {
            return password;
        }
    }

    getPassword2() {
        const password = document.getElementById("inputPassword2").value;

        if (password) {
            return password;
        }
    }

    redirect(url) {
        window.location.href = url;
    }
}
'use strict';

import { BadRequestError } from "../../admin/js/Errors/BadRequestError.js";
import { ConflictError } from "../../admin/js/Errors/ConflictError.js";
import { InternalServerError } from "../../admin/js/Errors/InternalServerError.js";
import { InstallStageTwoViewModel } from "../ViewModel/InstallStageTwoViewModel.js";

export class InstallStageTwoView {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config    
        this.viewModel = new InstallStageTwoViewModel(this.#config);

        this.checkAlreadyInstalled();
        this.ifStageOneDataIsEmpty();
    }

    async checkAlreadyInstalled() {
        let res = null;

        try {
            res = await this.viewModel.alreadyInstalled();
        } catch(error) {
            if (error instanceof InternalServerError) {
                this.alertDanger(error.message);
            } else if (error instanceof TypeError) {
                this.alertDanger("Network error. Failed to connect to the network.");
            } else {
                this.alertDanger(error.message);
            }
        }

        if (res.data.already_installed === true) {
            this.redirectToAlreadyInstalledPage();
        }
    }

    redirectToAlreadyInstalledPage() {
        window.location.href = "/already-installed";
    }

    async putDataToBackend() {
        let res = null;
        const stageOneData = this.getStageOneData();
        const siteTitle = stageOneData.site_title;
        const tagline = stageOneData.tagline;
        const firstName = this.getFirstName();
        const lastName = this.getLastName();
        const username = this.getUsername();
        const email = this.getEmail();
        const password1 = this.getPassword1();
        const password2 = this.getPassword2();

        try {
            res = await this.viewModel.putDataToBackend(
                siteTitle,
                tagline,
                firstName,
                lastName,
                username,
                email,
                password1,
                password2
            );
        } catch(error) {
            if (error instanceof InternalServerError) {
                this.alertDanger(error.message);
            } else if (error instanceof BadRequestError) {
                console.log(error.response);
                switch(error.response.error_code) {
                    case 31 :
                        this.alertDanger("Invalid data. Site title data is invalid.");
                        break;
                    case 32 :
                        this.alertDanger("Invalid data. Tagline data is invalid.");
                        break;
                    case 33 :
                        this.firstNameInputInvalid();
                        break;
                    case 34 :
                        this.usernameInputInvalid();
                        break;
                    case 35 :
                        this.usernameInputInvalid(error.response.error_message);
                        break;
                    case 36 :
                        this.emailInputInvalid();
                        break;
                    case 37 :
                        this.password1InputValid();
                        break;
                    case 38 :
                        this.password2InputInvalid();
                        break;
                    case 39 :
                        this.password1InputInvalid(error.response.error_message);
                        this.password2InputInvalid(error.response.error_message);
                        break;
                    default :
                        this.alertDanger("Error. Bad request.");
                        break;
                }
            } else if (error instanceof ConflictError) {
                this.alertDanger(error.response.error_message);
                this.redirectToDashboard();
            } else if (error instanceof TypeError) {
                this.alertDanger("Network error. Failed to connect to the network.");
            } else {
                this.alertDanger(error.message);
            }
        }

        if (res.response === true) {
            this.redirectToFinish();
        } else {
            this.alertDanger("Installation failed, please try again.");
        }
    }

    redirectToStageOne() {
        window.location.href = "/install/stage/1";
    }

    redirectToFinish() {
        window.location.href = "/install/finish";
    }

    redirectToDashboard() {
        window.location.href = "/admin";
    }

    ifStageOneDataIsEmpty() {
        const stageOneData = this.getStageOneData();

        if (stageOneData === null) {
            this.redirectToStageOne();
        }

        if (stageOneData.site_title === null || stageOneData.site_title === undefined) {
            this.redirectToStageOne();
        }

        if (stageOneData.tagline === null || stageOneData.tagline === undefined) {
            this.redirectToStageOne();
        }
    }

    getStageOneData() {
        let params = Object.fromEntries(new URL(window.location.href).searchParams);

        if (params === undefined) {
            return null;
        }

        if (params.data_from_stage_one === undefined) {
            return null;
        }

        const dataFromStageOne = this.dataFromStageOneDecoder(params.data_from_stage_one);
        const dataObject = this.queryStringToObject(dataFromStageOne);

        return dataObject;
    }

    dataFromStageOneDecoder(data) {
        const utf8decoder = new TextDecoder();
        const byteArray = new Uint8Array(data.split(",").map(Number));
        const decoded = utf8decoder.decode(byteArray);

        return decoded;
    }

    queryStringToObject(queryString) {
        const params = new URLSearchParams(queryString);

        return Object.fromEntries(params);
    }

    #validation() {
        let isValid = true;

        if (this.getFirstName() === null) {
            isValid = false;
            this.firstNameInputInvalid();
        } else {
            this.firstNameInputValid();
        }

        if (this.getUsername() === null) {
            isValid = false;
            this.usernameInputInvalid();
        } else {
            this.usernameInputValid();
        }

        if (this.getEmail() === null) {
            isValid = false;
            this.emailInputInvalid();
        } else {
            this.emailInputValid();
        }

        if (this.getPassword1() === null) {
            isValid = false;
            this.password1InputInvalid();
        } else {
            this.password1InputValid();
        }

        if (this.getPassword2() === null) {
            isValid = false;
            this.password2InputInvalid();
        } else {
            this.password2InputValid();
        }

        if (this.getPassword1() !== null && this.getPassword2() !== null) {
            if (this.getPassword1() !== this.getPassword2()) {
                isValid = false;
                this.password1InputInvalid("Password doesn't match.");
                this.password2InputInvalid("Password doesn't match.");
            } else {
                this.password1InputValid();
                this.password2InputValid();
            }
        }

        return isValid;
    }

    async submit() {
        if (this.#validation() === true) {
            this.buttonLoading();
            await this.putDataToBackend();
            this.buttonNormal();
        }
    }

    firstNameInputInvalid(message) {
        if (message != null) {
            document.getElementById("firstNameInvalidFeedback").innerText = message;
        }
        
        document.getElementById("firstName").classList.add('is-invalid');
    }

    usernameInputInvalid(message) {
        if (message != null) {
            document.getElementById("usernameInvalidFeedback").innerText = message;
        }
        
        document.getElementById("username").classList.add('is-invalid');
    }

    emailInputInvalid(message) {
        if (message != null) {
            document.getElementById("emailInvalidFeedback").innerText = message;
        }
        
        document.getElementById("email").classList.add('is-invalid');
    }

    password1InputInvalid(message) {
        if (message != null) {
            document.getElementById("password1InvalidFeedback").innerText = message;
        }
        
        document.getElementById("password1").classList.add('is-invalid');
    }

    password2InputInvalid(message) {
        if (message != null) {
            document.getElementById("password2InvalidFeedback").innerText = message;
        }
        
        document.getElementById("password2").classList.add('is-invalid');
    }

    firstNameInputValid() {
        document.getElementById("firstName").classList.remove('is-invalid');
    }

    usernameInputValid() {
        document.getElementById("username").classList.remove('is-invalid');
    }

    emailInputValid() {
        document.getElementById("email").classList.remove('is-invalid');
    }

    password1InputValid() {
        document.getElementById("password1").classList.remove('is-invalid');
    }

    password2InputValid() {
        document.getElementById("password2").classList.remove('is-invalid');
    }

    getFirstName() {
        let data = document.getElementById("firstName").value;

        if (data === "") {
            data = null;
        }

        return data;
    }

    getLastName() {
        let data = document.getElementById("lastName").value;

        if (data === "") {
            data = null;
        }

        return data;
    }

    getUsername() {
        let data = document.getElementById("username").value;

        if (data === "") {
            data = null;
        }

        return data;
    }

    getEmail() {
        let data = document.getElementById("email").value;

        if (data === "") {
            data = null;
        }

        return data;
    }

    getPassword1() {
        let data = document.getElementById("password1").value;

        if (data === "") {
            data = null;
        }

        return data;
    }

    getPassword2() {
        let data = document.getElementById("password2").value;

        if (data === "") {
            data = null;
        }

        return data;
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
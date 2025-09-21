'use strict';

import { InternalServerError } from "../../admin/js/Errors/InternalServerError.js";
import { InstallStageOneViewModel } from "../ViewModel/InstallStageOneViewModel.js";

export class InstallStageOneView {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new InstallStageOneViewModel(this.#config);

        this.checkAlreadyInstalled();
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

    #validation() {
        let ret = true;

        if (this.getSiteTitleValue() === "") {
            ret = false;
            this.siteTitleInputInvalid();
        } else {
            this.siteTitleInputValid();
        }
        
        if (this.getTaglineValue() === "") {
            ret = false;
            this.taglineInputInvalid();
        } else {
            this.taglineInputValid();
        }

        return ret;
    }

    submit() {
        if (this.#validation() === true) {
            const queryString = this.stageOneDataBuilder(this.getSiteTitleValue(), this.getTaglineValue());

            window.location.href = "/install/stage/2?data_from_stage_one=" + this.stageOneDataEncoder(queryString);
        }
    }

    stageOneDataEncoder(queryString) {
        const utf8encoder = new TextEncoder();

        return utf8encoder.encode(queryString);
    }

    stageOneDataBuilder(siteTitle, tagline) {
        const queryString = "data_from=" + encodeURIComponent("stage 1") + 
                            "&site_title=" + encodeURIComponent(siteTitle) +
                            "&tagline=" + encodeURIComponent(tagline);
        
        return queryString;
    }

    getSiteTitleValue() {
        const title = document.getElementById("siteTitle").value;

        return title;
    }

    getTaglineValue() {
        const tagline = document.getElementById("tagline").value;

        return tagline;
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
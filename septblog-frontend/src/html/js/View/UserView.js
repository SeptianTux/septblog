'use strict';

import { HttpError } from "../../admin/js/Errors/HttpError.js";
import { NotFoundError } from "../../admin/js/Errors/NotFoundError.js";
import { Page } from "../Utils/Page.js";
import { UserViewModel } from "../ViewModel/UserViewModel.js";

export class UserView {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new UserViewModel(this.#config);

        this.setProfile();

        this.#setSiteTitle();
        this.#setNavbarBrand();
        this.#setCopyright();
    }

    async #setSiteTitle() {
        await Page.setSiteTitle();
    }

    async #setNavbarBrand() {
        await Page.setNavbarBrand();
    }

    async #setCopyright() {
        await Page.setCopyright();
    }

    getUsernameFromUrl() {
        const pathSegments = window.location.pathname.split("/").filter(segment => segment);

        if(pathSegments[1] === null || pathSegments[1] === undefined) {
            return null;
        }

        return pathSegments[1];
    }

    setProfileImage(url) {
        document.getElementById("profileImage").src = url;
    }

    setProfileName(name) {
        const n = '<h2>' + name + '</h2>';
        document.getElementById("profileName").replaceChildren();
        document.getElementById("profileName").insertAdjacentHTML("afterbegin", n);
    }

    setProfileAbout(about) {
        document.getElementById("profileAbout").replaceChildren();
        document.getElementById("profileAbout").insertAdjacentHTML("afterbegin", about);
    }

    async setProfile() {
        const username = this.getUsernameFromUrl();

        if (username == null) {
            this.redirectToNotFoundPage();
        }

        const profileData = await this.getProfileDataFromBackend(username);
        const fullName = profileData.last_name === null ? profileData.first_name : profileData.first_name + " " + profileData.last_name;
        
        this.setProfileImage(profileData.avatar);
        this.setProfileName(fullName);

        if (profileData.about !== null) {
            this.setProfileAbout(profileData.about);
        } else {
            this.setProfileAbout("");
        }
    }

    async getProfileDataFromBackend(username) {
        let res = null;

        try {
            res = await this.viewModel.getProfileDataFromBackend(username);
        } catch(error) {
            if (error instanceof NotFoundError) {
                this.redirectToNotFoundPage();
            } else if (error instanceof HttpError) {
                this.alertDanger(error.message);
            } else {
                this.alertDanger(error.message);
            }
        }

        if (res.response === true) {
            return res.data;
        } else {
            this.alertDanger("Failed to get profile data.");
        }
    }

    redirectToNotFoundPage() {
        window.location.href = "/404";
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
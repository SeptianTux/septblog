'use strict';

import { BadRequestError } from "../Errors/BadRequestError.js";
import { HttpError } from "../Errors/HttpError.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { ProfileViewModel } from "../ViewModel/ProfileViewModel.js";
import { Page } from "../Utils/Page.js";
import { User } from "../Utils/User.js";

export class ProfileView {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new ProfileViewModel(this.#config);

        this.#checkCredentials();
        
        this.#formEnterKeyListener();
        this.setProfileData();
        this.imageUploadListener();
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

    #validation() {
        let isValid = true;

        if (document.getElementById("firstName").value === "") {
            isValid = false;
            this.firstNameInputInvalid();
        } else {
            this.firstNameInputValid();
        }
        
        if (document.getElementById("about").value === "") {
            isValid = false;
            this.aboutInputInvalid();
        } else {
            this.aboutInputValid();
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

    async getProfileData() {
        let res = null;

        try {
            res = await this.viewModel.getProfileData();
        } catch(error) {
            if (error instanceof TypeError) {
                this.alertDanger("Failed to connect to the network.");
            } else if (error instanceof UnauthorizedError) {
                this.redirectToLoginPage();
            } else if (error instanceof HttpError) {
                this.alertDanger("HTTP error. " + error.message + ".");
            } else {
                this.alertDanger(error.message);
            }
        }

        return res;
    }

    async setProfileData() {
        const data = await this.getProfileData();

        this.setAvatar(data.data.avatar);
        this.setFirstName(data.data.first_name);
        this.setLastName(data.data.last_name);
        this.setUsername(data.data.username);
        this.setEmail(data.data.email);
        this.setAbout(data.data.about);
    }

    async putProfileData(
        avatar,
        firstName,
        lastName,
        about
    ) {
        let res = null;

        try {
            res = await this.viewModel.putProfileData(
                avatar,
                firstName,
                lastName,
                about
            );
        } catch(error) {
            if (error instanceof TypeError) {
                this.alertDanger("Failed to connect to the network.");
            } else if (error instanceof UnauthorizedError) {
                this.redirectToLoginPage();
            } else if (error instanceof HttpError) {
                this.alertDanger(error.message);
            } else if (error instanceof BadRequestError) {
                const res = await error.response;
                this.alertDanger(res.error_message);
            } else {
                this.alertDanger(error.message);
            }
        }

        return res;
    }

    async submit() {
        const validation = this.#validation();

        this.#buttonLoading();

        if (validation === true) {
            const avatar = this.getAvatar();
            const firstName = this.getFirstName();
            const lastName = this.getLastName();
            const about = this.getAbout();

            const res = await this.putProfileData(
                avatar,
                firstName,
                lastName,
                about
            );

            if (res !== null) {
                if (res.response === true) {
                    this.alertSuccess("Data changed.");
                } else {
                    this.alertSuccess("Failed to change data.");
                }
            }
        }

        this.#buttonNormal();
    }

    addImageUploadProgressbar() {
        const imageUploadProgressbar = '<div id="removableImageUploadProgressbar" class="upload-progressbar">' +
                                            '<div id="uploadProgressbar" class="progress" role="progressbar" aria-valuenow="0" aria-valuemin="0" aria-valuemax="100">' +
                                                '<div id="imageUploadProgressbar" class="progress-bar progress-bar-striped progress-bar-animated" style="width: 0%">' +
                                                    '0%' +
                                                '</div>' +
                                            '</div>' +
                                        '</div>';

        document.getElementById("adminProfileChangeValue").insertAdjacentHTML("beforeend", imageUploadProgressbar);
    }

    removeImageUploadProgressbar() {
        const progressbar = document.getElementById("removableImageUploadProgressbar");

        if (progressbar !== null) {
            progressbar.remove();
        }
    }
    
    updateImageUploadProgressbar(value) {
        let progressBar = document.getElementById('imageUploadProgressbar');
        progressBar.style.width = value + '%';
        progressBar.textContent = value + '%';
    }

    imageUploadListener() {
        document.getElementById('formFile').addEventListener('change', function () {
            const file = this.files[0];
            if (!file) return;

            // Put the file into FormData for sending
            const formData = new FormData();
            formData.append('image', file);

            // Create request
            const xhr = new XMLHttpRequest();
            try {
                main.view.viewModel.imageUpload(xhr);
            } catch(error) {
                if (error instanceof UnauthorizedError) {
                    main.view.redirectToLoginPage();
                } else {
                    main.view.alertClose();
                    main.view.alertDanger(error.message);
                }
            }

            main.view.addImageUploadProgressbar();

            // Track upload progress
            xhr.upload.addEventListener('progress', function (e) {
                if (e.lengthComputable) {
                    const percent = (e.loaded / e.total) * 100;
                    main.view.updateImageUploadProgressbar(percent);
                }
            });

            // When upload finishes
            xhr.addEventListener('load', function () {
                if (xhr.status === 201) {
                    let responseData = null;

                    try {
                        responseData = JSON.parse(xhr.responseText);
                    } catch (e) {
                        main.view.alertDanger("Failed to parse data.");
                    }

                    main.view.setAvatar(responseData.url);
                    main.view.removeImageUploadProgressbar();
                } else {
                    main.view.alertDanger("Failed to upload an image. The server returned " + xhr.status + ".");
                }
            });

            // Error handling
            xhr.addEventListener('error', function () {
                main.view.alertDanger("Failed to upload an image.");
                main.view.removeImageUploadProgressbar();
            });

            // Send the request
            xhr.send(formData);
        });
    }

    firstNameInputInvalid(message) {
        if (message != null) {
            document.getElementById("firstNameInvalidFeedback").innerText = message;
        }
        
        document.getElementById("firstName").classList.add('is-invalid');
    }

    lastNameInputInvalid(message) {
        if (message != null) {
            document.getElementById("lastNameInvalidFeedback").innerText = message;
        }
        
        document.getElementById("lastName").classList.add('is-invalid');
    }

    aboutInputInvalid(message) {
        if (message != null) {
            document.getElementById("aboutInvalidFeedback").innerText = message;
        }
        
        document.getElementById("about").classList.add('is-invalid');
    }

    firstNameInputValid() {
        document.getElementById("firstName").classList.remove('is-invalid');
    }

    lastNameInputValid() {
        document.getElementById("lastName").classList.remove('is-invalid');
    }

    aboutInputValid() {
        document.getElementById("about").classList.remove('is-invalid');
    }

    saveButtonLoading() {
        return this.#buttonLoading("saveButton");
    }

    saveButtonNormal() {
        return this.#buttonNormal("saveButton", "Save");
    }

    setAvatar(src) {
        document.getElementById("avatar").src = src;
    }

    setFirstName(value) {
        document.getElementById("firstName").value = value;
    }

    setLastName(value) {
        document.getElementById("lastName").value = value;
    }

    setUsername(value) {
        document.getElementById("username").value = value;
    }

    setEmail(value) {
        document.getElementById("email").value = value;
    }

    setAbout(value) {
        document.getElementById("about").value = value;
    }

    getAvatar() {
        const url = new URL(document.getElementById("avatar").src);

        return url.pathname;
    }

    getFirstName() {
        return document.getElementById("firstName").value;
    }

    getLastName() {
        return document.getElementById("lastName").value;
    }

    getAbout() {
        return document.getElementById("about").value;
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

    redirectToLoginPage() {
        const path = "/admin/profile";
        const redirectTo = encodeURIComponent(path);

        window.location.href = "/admin/login?redirect-to=" + redirectTo;
    }
}
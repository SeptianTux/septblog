'use strict';

import { UsersViewModel } from "../ViewModel/UsersViewModel.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { HttpError } from "../Errors/HttpError.js";
import { Page } from "../Utils/Page.js";
import { User } from "../Utils/User.js";
import { ForbiddenError } from "../Errors/ForbiddenError.js";

export class UsersView {
    #config = null;
    viewModel = null;

    #markedUserIdForUserDeletion = null;
    #markedUserIdForUserSuspend = null;
    #markedUserIdForUserActivate = null;

    #itsLoadingUsersNow = false;
    #isBackendHasMore = true;
    #page = 1;

    constructor(config) {
        this.#config = config;
        this.viewModel = new UsersViewModel(this.#config);

        this.#checkPrivilage();

        this.#init();

        this.#infiniteScrollListener();
        this.#windowsResizeListener();
        this.#setPageTitle();
        this.#setNavbarBrand();
        this.#setCopyright();
    }

    async #init() {
        this.addInfiniteScrollSpinners();
        this.setItsLoadingUsersNow(true);

        await this.#loadUsers();
        await this.#itsShouldLoadMore();

        this.setItsLoadingUsersNow(false);
        this.removeInfiniteScrollSpinners();
        this.#setLoggedInAs();

        this.#administratorsNavMenu();
    }

    async #administratorsNavMenu() {
        await User.administratorsNavMenu();
    }

    async #checkPrivilage() {
        let userLevel = null;
        
        try {
            userLevel = await User.getUserLevel();
        } catch(error) {
            if (error instanceof UnauthorizedError) {
                this.#redirectToLoginPage();
            } else {
                this.alertDanger(error.message);
            }
        }

        if (userLevel === 'user') {
            this.#redirectToDashboard();
        }
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

    async getUsersDataFromBackend(page) {
        let res = null;

        try {
            res = await this.viewModel.getUsersDataFromBackend(page);
        } catch(error) {
            if (error instanceof TypeError) {
                this.alertDanger("Failed to connect to the network.");
            } else if (error instanceof UnauthorizedError) {
                this.#redirectToLoginPage();
            } else if (error instanceof ForbiddenError) {
                this.#redirectToDashboard();
                //console.log("Forbidden.");
            } else if (error instanceof HttpError) {
                this.alertDanger(error.message);
            } else {
                this.alertDanger(error.message);
            }
        }

        return res;
    }

    getPage() {
        return this.#page;
    }

    increasePage() {
        this.#page++;
    }

    setIsBackendHasMore(hasMore) {
        this.#isBackendHasMore = hasMore;
    }

    getIsBackendHasMore() {
        return this.#isBackendHasMore;
    }

    setItsLoadingUsersNow(loading) {
        this.#itsLoadingUsersNow = loading;
    }

    getItsLoadingUsersNow() {
        return this.#itsLoadingUsersNow;
    }

    setIsBackendHasMore(value) {
        this.#isBackendHasMore = value;
    }

    async #loadUsers() {
        const getUsers = await this.getUsersDataFromBackend(this.getPage());

        this.increasePage();

        if (getUsers) {
            if (!getUsers.has_more) {
                this.setIsBackendHasMore(false);
            } else if(getUsers.has_more === true) {
                for (let i = 0; i < getUsers.data.length; i++) {
                    const fullName = getUsers.data[i].last_name === null ? getUsers.data[i].first_name : getUsers.data[i].first_name + " " + getUsers.data[i].last_name;
                    const date = new Date(getUsers.data[i].created * 1000);
                    const created = date.getDate() + "/" + (date.getMonth()+1) + "/" + date.getFullYear() + " " + date.getHours() + ":" + date.getMinutes() + ":" + date.getSeconds();
                    const userLevel = getUsers.data[i].level > 0 ? "Standard User" : "Administrator";
                    const avatar = getUsers.data[i].avatar === null ? "/uploads/user.png" : getUsers.data[i].avatar;
                    const status = getUsers.data[i].status === 0 ? "Active" : "Suspended";
                    const suspendOrActivate = getUsers.data[i].status === 0 ? 
                                                    '<li id="suspendOrActivate-' + getUsers.data[i].id + '"><a onclick="main.view.markUserIdForSuspend(' + getUsers.data[i].id + ');" class="dropdown-item" data-bs-toggle="modal" data-bs-target="#userSuspendModal" href="#">Suspend</a></li>'
                                                    :
                                                    '<li id="suspendOrActivate-' + getUsers.data[i].id + '"><a onclick="main.view.markUserIdForActivation(' + getUsers.data[i].id + ');" class="dropdown-item" data-bs-toggle="modal" data-bs-target="#userActivateModal" href="#">Activate</a></li>';
                    const tr = '<tr id="row-' + getUsers.data[i].id + '">' +
                                    '<td>' +
                                        '<table>' +
                                            '<tr>' +
                                                '<td>' +
                                                    '<img src="' + avatar + '" class="img-thumbnail users-pp" alt="...">' +
                                                '</td>' +
                                                '<td class="users-name">' +
                                                    '<div id="fullName">' + fullName + '</div>' +
                                                    '<div class="text-muted" id="username"><small>@' + getUsers.data[i].username + '</small></div>' +
                                                '</td>' +
                                            '</tr>' +
                                        '</table>' +
                                    '</td>' +
                                    '<td>' + userLevel +'</td>' +
                                    '<td>' + getUsers.data[i].articles + '</td>' +
                                    '<td id="status-' + getUsers.data[i].id + '">' + status + '</td>' +
                                    '<td style="">' + created + '</td>' +
                                    '<td>' +
                                        '<div class="dropdown">' +
                                            '<button class="btn btn-primary btn-sm dropdown-toggle" type="button" data-bs-toggle="dropdown" aria-expanded="false">' +
                                                'Actions' +
                                            '</button>' +
                                            '<ul class="dropdown-menu">' +
                                                suspendOrActivate +
                                                '<li><a onclick="main.view.markUserIdForDeletion(\'' + getUsers.data[i].id + '\');" class="dropdown-item" data-bs-toggle="modal" data-bs-target="#userDeletionModal" href="#">Delete</a></li>' +
                                            '</ul>' +
                                        '</div>' +
                                    '</td>' +
                                '</tr>';
                    
                    document.getElementById("usersTable").insertAdjacentHTML("beforeend", tr);
                }
            }
        } else {
            this.setIsBackendHasMore(false);
        }
    }

    #shouldLoadMore() {
        return document.body.scrollHeight <= window.innerHeight;
    }

    async #itsShouldLoadMore() {
        while(this.#shouldLoadMore() && this.getIsBackendHasMore()) {
            this.setItsLoadingUsersNow();
            this.addInfiniteScrollSpinners();
            await this.#loadUsers();
            this.removeInfiniteScrollSpinners();
            this.setItsLoadingUsersNow(false);
        }
    }

    async #loadUsersForInfiniteScrollListerner() {
        this.setItsLoadingUsersNow(true);
        this.addInfiniteScrollSpinners();
        await this.#loadUsers();
        this.removeInfiniteScrollSpinners();
        this.setItsLoadingUsersNow(false);
    }

    #infiniteScrollListener() {
        window.addEventListener("scroll", () => {
            if (window.innerHeight + window.scrollY >= document.getElementById("usersTable").offsetHeight + 80) {
                if ((!this.getItsLoadingUsersNow()) && this.getIsBackendHasMore()) {
                    this.#loadUsersForInfiniteScrollListerner();
                }
            }
        });
    }

    #windowsResizeListener() {
        window.addEventListener("resize", () => {
            if (this.#shouldLoadMore() && !this.getItsLoadingUsersNow() && this.getIsBackendHasMore()) {
                this.#itsShouldLoadMore();
            }
        });
    }

    markUserIdForDeletion(userId) {
        this.#markedUserIdForUserDeletion = userId;
    }

    unmarkUserIdForDeletion() {
        this.#markedUserIdForUserDeletion = null;
    }

    markUserIdForSuspend(userId) {
        this.#markedUserIdForUserSuspend = userId;
    }

    unmarkUserIdForSuspend() {
        this.#markedUserIdForUserSuspend = null;
    }

    markUserIdForActivation(userId) {
        this.#markedUserIdForUserActivate = userId;
    }

    unmarkUserIdForActivate() {
        this.#markedUserIdForUserActivate = null;
    }

    deleteUser() {
        if (this.#markedUserIdForUserDeletion != null) {
            this.hideUserDeletionModalDialog();
            this.removeUserAndTableRow(this.#markedUserIdForUserDeletion);
            this.unmarkUserIdForDeletion();
        } else {
            this.hideUserDeletionModalDialog();
        }
    }

    cancelDeleteUser() {
        this.unmarkUserIdForDeletion();
    }

    suspendUser() {
        if (this.#markedUserIdForUserSuspend != null) {
            this.hideUserSuspendModalDialog();
            this.setUserStatusToSuspend(this.#markedUserIdForUserSuspend);

            const link = '<a onclick="main.view.markUserIdForActivation(' + this.#markedUserIdForUserSuspend + ');" class="dropdown-item" data-bs-toggle="modal" data-bs-target="#userActivateModal" href="#">Activate</a>';

            document.getElementById("suspendOrActivate-" + this.#markedUserIdForUserSuspend).innerHTML = "";
            document.getElementById("suspendOrActivate-" + this.#markedUserIdForUserSuspend).insertAdjacentHTML("beforeend", link);

            this.unmarkUserIdForSuspend();
        } else {
            this.hideUserSuspendModalDialog();
        }
    }

    cancelSuspendUser() {
        this.unmarkUserIdForSuspend();
    }

    activateUser() {
        if (this.#markedUserIdForUserActivate != null) {
            this.hideUserActivateModalDialog();
            this.setUserStatusToActive(this.#markedUserIdForUserActivate);

            const link = '<a onclick="main.view.markUserIdForSuspend(' + this.#markedUserIdForUserActivate + ');" class="dropdown-item" data-bs-toggle="modal" data-bs-target="#userSuspendModal" href="#">Suspend</a>';

            document.getElementById("suspendOrActivate-" + this.#markedUserIdForUserActivate).innerHTML = "";
            document.getElementById("suspendOrActivate-" + this.#markedUserIdForUserActivate).insertAdjacentHTML("beforeend", link);

            this.unmarkUserIdForActivate();
        } else {
            this.hideUserActivateModalDialog();
        }
    }

    cancelActivateUser() {
        this.unmarkUserIdForActivate();
    }

    async removeUserAndTableRow(userId) {
        let res = null;

        try {
            res = await this.viewModel.deleteUser(userId);
        } catch(error) {
            if (error instanceof UnauthorizedError) {
                this.#redirectToLoginPage();
            } else if (error instanceof TypeError) {
                this.alertDanger("Failed to connect to the network.");
            } else {
                this.alertDanger(error.message);
            }
        }

        if (res.response === true) {
            const row = document.getElementById("row-" + userId);

            if (row)
                row.remove();

            this.alertSuccess("User deleted.")
        } else {
            this.alertDanger("Failed to delete user.");
        }
    }

    async setUserStatusToSuspend(userId) {
        let res = null;

        try {
            res = await this.viewModel.suspendUser(userId);
        } catch(error) {
            if (error instanceof UnauthorizedError) {
                this.#redirectToLoginPage();
            } else if (error instanceof TypeError) {
                this.alertDanger("Failed to connect to the network.");
            } else if (error instanceof ForbiddenError) {
                this.#redirectToDashboard();
                //console.log("Forbidden.");
            }
            else {
                this.alertDanger(error.message);
            }
        }

        if (res.response === true) {
            const status = document.getElementById("status-" + userId);

            if (status)
                status.innerText = "Suspended";

            this.alertSuccess("User suspended.")
        } else {
            this.alertDanger("Failed to suspend user.");
        }
    }

    async setUserStatusToActive(userId) {
        let res = null;

        try {
            res = await this.viewModel.activateUser(userId);
        } catch(error) {
            if (error instanceof UnauthorizedError) {
                this.#redirectToLoginPage();
            } else if (error instanceof TypeError) {
                this.alertDanger("Failed to connect to the network.");
            } else if (error instanceof ForbiddenError) {
                this.#redirectToDashboard();
                //console.log("Forbidden.");
            } else {
                this.alertDanger(error.message);
            }
        }

        if (res.response === true) {
            const status = document.getElementById("status-" + userId);

            if (status)
                status.innerText = "Active";

            this.alertSuccess("User activated.")
        } else {
            this.alertDanger("Failed to activate user.");
        }
    }

    isInfiniteScrollSpinnersExist() {
        const spinners = document.getElementById("usersInfiniteScrollSpinners");

        if (spinners == null)
            return false;

        return true;
    }

    addInfiniteScrollSpinners() {
        const spinners = '<tr id="usersInfiniteScrollSpinners">' +
                            '<th scope="row">' +
                                '<div class="d-flex justify-content-center">' +
                                    '<div class="spinner-border" role="status">' +
                                        '<span class="visually-hidden">Loading...</span>' +
                                    '</div>' +
                                '</div>' +
                            '</th>' +
                            '<td>' +
                                '<div class="d-flex justify-content-center">' +
                                    '<div class="spinner-border" role="status">' +
                                        '<span class="visually-hidden">Loading...</span>' +
                                    '</div>' +
                                '</div>' +
                            '</td>' +
                            '<td>' +
                                '<div class="d-flex justify-content-center">' +
                                    '<div class="spinner-border" role="status">' +
                                        '<span class="visually-hidden">Loading...</span>' +
                                    '</div>' +
                                '</div>' +
                            '</td>' +
                            '<td>' +
                                '<div class="d-flex justify-content-center">' +
                                    '<div class="spinner-border" role="status">' +
                                        '<span class="visually-hidden">Loading...</span>' +
                                    '</div>' +
                                '</div>' +
                            '</td>' +
                            '<td>' +
                                '<div class="d-flex justify-content-center">' +
                                    '<div class="spinner-border" role="status">' +
                                        '<span class="visually-hidden">Loading...</span>' +
                                    '</div>' +
                                '</div>' +
                            '</td>' +
                            '<td>' +
                                '<div class="d-flex justify-content-center">' +
                                    '<div class="spinner-border" role="status">' +
                                        '<span class="visually-hidden">Loading...</span>' +
                                    '</div>' +
                                '</div>' +
                            '</td>' +
                        '</tr>';
        
        
        if (!this.isInfiniteScrollSpinnersExist()) {
            document.getElementById("usersTable").insertAdjacentHTML("beforeend", spinners);

            return true;
        }

        return false;
    }

    removeInfiniteScrollSpinners() {
        if (this.isInfiniteScrollSpinnersExist()) {
            document.getElementById("usersInfiniteScrollSpinners").remove();

            return true;
        }

        return false;
    }

    hideUserDeletionModalDialog() {
        const modal = document.getElementById('userDeletionModal');
        const modalInstance = bootstrap.Modal.getOrCreateInstance(modal);
        
        modalInstance.hide();
    }

    showUserDeletionModalDialog() {
        const modal = document.getElementById('userDeletionModal');
        const modalInstance = bootstrap.Modal.getOrCreateInstance(modal);
        
        modalInstance.show();
    }

    hideUserSuspendModalDialog() {
        const modal = document.getElementById('userSuspendModal');
        const modalInstance = bootstrap.Modal.getOrCreateInstance(modal);
        
        modalInstance.hide();
    }

    showUserSuspendModalDialog() {
        const modal = document.getElementById('userSuspendModal');
        const modalInstance = bootstrap.Modal.getOrCreateInstance(modal);
        
        modalInstance.show();
    }

    hideUserActivateModalDialog() {
        const modal = document.getElementById('userActivateModal');
        const modalInstance = bootstrap.Modal.getOrCreateInstance(modal);
        
        modalInstance.hide();
    }

    showUserActivateModalDialog() {
        const modal = document.getElementById('userActivateModal');
        const modalInstance = bootstrap.Modal.getOrCreateInstance(modal);
        
        modalInstance.show();
    }

    #redirectToLoginPage() {
        const path = "/admin/users";
        const redirectTo = encodeURIComponent(path);

        window.location.href = "/admin/login?redirect-to=" + redirectTo;
    }

    #redirectToDashboard() {
        window.location.href = "/admin";
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
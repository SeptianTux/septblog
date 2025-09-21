'use strict';

import { HttpError } from "../Errors/HttpError.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { VisitorsViewModel } from "../ViewModel/VisitorsViewModel.js";
import { Page } from "../Utils/Page.js";
import { User } from "../Utils/User.js";

export class VisitorsView {
    #config = null;
    viewModel = null;

    #itsLoadingVisitorsNow = false;
    #isBackendHasMore = true;
    #page = 1;

    constructor(config) {
        this.#config = config;
        this.viewModel = new VisitorsViewModel(this.#config);

        this.#init();

        this.#infiniteScrollListener();
        this.#windowsResizeListener();
        this.#setLoggedInAs();
        this.#setPageTitle();
        this.#setNavbarBrand();
        this.#setCopyright();

        this.#administratorsNavMenu();
    }

    async #administratorsNavMenu() {
        await User.administratorsNavMenu();
    }

    async #init() {
        this.setItsLoadingVisitorNow(true);

        await this.#loadVisitors();
        await this.#itsShouldLoadMore();

        this.setItsLoadingVisitorNow(false);
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

    copyToClipboard(value) {
        navigator.clipboard.writeText(value);
        /*
        .then(() => {
            alert("Copied to clipboard: " + text);
        })
        .catch(err => {
            console.error("Failed to copy: ", err);
        });
        */
    }

    copyUserAgentToClipboard(id) {
        const userAgent = document.getElementById(id).value;
        
        this.copyToClipboard(userAgent);
    }

    copyRefererToClipboard(id) {
        const referer = document.getElementById(id).value;

        this.copyToClipboard(referer);
    }

    async #loadVisitors() {
        const getVisitorsData = await this.getVisitorsDataFromBackend(this.getPage());

        this.increasePage();

        if (getVisitorsData.has_more === true) {
            const table = '<div class="table-responsive">' +
                                '<table id="tableVisitors" class="table table-striped table-bordered table-hover table-visitors">' +
                                    '<thead>' +
                                        '<tr>' +
                                            '<th>Article</th>' +
                                            '<th>User Agent</th>' +
                                            '<th>IP Adress</th>' +
                                            '<th>Referer</th>' +
                                            '<th>Date/Time</th>' +
                                        '</tr>' +
                                    '</thead>' +
                                    '<tbody id="visitorsTableData">' +
                                    '</tbody>' +
                                '</table>' +
                            '</div>';
            document.getElementById("cardBody").insertAdjacentHTML("beforeend", table);
            
            for (let i = 0; i < getVisitorsData.data.length; i++) {
                const date = new Date(getVisitorsData.data[i].visited_at * 1000);
                const visitedAt = date.getDate() + '/' + (date.getMonth()+1) + '/' + date.getFullYear() + ' ' + date.getHours() + ':' + date.getMinutes() + ':' + date.getSeconds();
                const tr = '<tr>' +
                                '<td>' +
                                    '<a target="_blank" href="/article/' + getVisitorsData.data[i].article_id + '">' + getVisitorsData.data[i].article_title + '</a>' +
                                '</td>' +
                                '<td>' +
                                    '<div class="input-group mb-3">' +
                                        '<input id="userAgent-' + getVisitorsData.data[i].article_id + '" type="text" class="form-control" placeholder="User Agent" aria-label="User Agent" aria-describedby="button-addon2" value="' + getVisitorsData.data[i].user_agent + '">' +
                                        '<button onClick="main.view.copyUserAgentToClipboard(\'userAgent-' + getVisitorsData.data[i].article_id + '\')" class="btn btn-outline-secondary" type="button" id="button-addon2"><i class="fa-solid fa-clipboard"></i></button>' +
                                    '</div>' +
                                '</td>' +
                                '<td>' +
                                    getVisitorsData.data[i].ip_address +
                                '</td>' +
                                '<td>' +
                                    '<div class="input-group mb-3">' +
                                        '<input id="referer-' + getVisitorsData.data[i].article_id + '" type="text" class="form-control" placeholder="User Agent" aria-label="User Agent" aria-describedby="button-addon2" value="' + getVisitorsData.data[i].referer +'">' +
                                        '<button onClick="main.view.copyRefererToClipboard(\'referer-' + getVisitorsData.data[i].article_id + '\')" class="btn btn-outline-secondary" type="button" id="button-addon2"><i class="fa-solid fa-clipboard"></i></button>' +
                                    '</div>' +
                                '</td>' +
                                '<td>' +
                                    visitedAt +
                                '</td>' +
                            '</tr>';
                
                document.getElementById("visitorsTableData").insertAdjacentHTML("beforeend", tr);
            }
        } else if (getVisitorsData.has_more === false) {
            if (this.getPage() === 2) {
                const div = '<div class="visitors-no-visitors-warning text-muted">There are no visitors yet.</div>';
                document.getElementById("cardBody").insertAdjacentHTML("beforeend", div);
            }

            this.setIsBackendHasMore(false);
        }
    }

    async #loadVisitorsForInfiniteScrollListerner() {
        this.setItsLoadingVisitorNow(true);
        await this.#loadVisitors();
        this.setItsLoadingVisitorNow(false);
    }

    #infiniteScrollListener() {
        window.addEventListener("scroll", () => {
            if (window.innerHeight + window.scrollY >= document.getElementById("tableVisitors").offsetHeight + 80) {
                if ((!this.getItsLoadingVisitorNow()) && this.getIsBackendHasMore()) {
                    this.#loadVisitorsForInfiniteScrollListerner();
                }
            }
        });
    }

    #windowsResizeListener() {
        window.addEventListener("resize", () => {
            if (this.#shouldLoadMore() && !this.getItsLoadingVisitorNow() && this.getIsBackendHasMore()) {
                this.#itsShouldLoadMore();
            }
        });
    }

    #shouldLoadMore() {
        return document.body.scrollHeight <= window.innerHeight;
    }

    async #itsShouldLoadMore() {
        while(this.#shouldLoadMore() && this.getIsBackendHasMore()) {
            this.setItsLoadingVisitorNow(true);
            await this.#loadVisitors();
            this.setItsLoadingVisitorNow(false);
        }
    }

    async getVisitorsDataFromBackend(page) {
        let res = null;

        try {
            res = await this.viewModel.getVisitorsDataFromBackend(page);
        } catch(error) {
            if (error instanceof UnauthorizedError) {
                this.#redirectToLoginPage();
            } else if(error instanceof HttpError) {
                this.alertDanger("HTTP error. " + error.message + ".");
            } else if (error instanceof TypeError) {
                this.alertDanger("Failed to connect to the network.");
            } else {
                this.alertDanger(error.message);
            }
        }

        return res;
    }

    setItsLoadingVisitorNow(value) {
        this.#itsLoadingVisitorsNow = value;
    }

    getItsLoadingVisitorNow() {
        return this.#itsLoadingVisitorsNow;
    }

    setIsBackendHasMore(value) {
        this.#isBackendHasMore = value;
    }

    getIsBackendHasMore() {
        return this.#isBackendHasMore;
    }

    getPage() {
        return this.#page;
    }

    increasePage() {
        this.#page++;
    }

    addInfiniteScrollSpinners() {
        let spinners = '<tr id="visitorsInfiniteScrollSpinners">' +
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

        if (!this.isInfiniteScrollSpinnerExist()) {
            document.getElementById("visitorsTableData").insertAdjacentHTML("beforeend", spinners);
            
            return true;
        }

        return false;
    }

    removeInfiniteScrollSpinners() {
        if (this.isInfiniteScrollSpinnerExist()) {
            document.getElementById("visitorsInfiniteScrollSpinners").remove();

            return true;
        }

        return false;
    }

    isInfiniteScrollSpinnerExist() {
        let spinner = document.getElementById("visitorsInfiniteScrollSpinners");

        if (spinner === null) {
            return false;
        }

        return true;
    }

    #redirectToLoginPage() {
        const path = "/admin/visitors";
        const redirectTo = encodeURIComponent(path);

        window.location.href = "/admin/login?redirect-to=" + redirectTo;
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
'use strict';

import { HttpError } from "../Errors/HttpError.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { ArticlesViewModel } from "../ViewModel/ArticlesViewModel.js";
import { Page } from "../Utils/Page.js";
import { User } from "../Utils/User.js";

export class ArticlesView {
    viewModel = null;
    #config = null;

    #itsLoadingArticlesNow = false;
    #infiniteScrollPage = 0;
    #isBackendHasMoreContents = true;
    #markArticleForMoveToTrash = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new ArticlesViewModel(this.#config);

        this.#checkCredentials();
        
        this.#init();
        this.#windowsResizeListener();
        this.#infiniteScrollListener();
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

    async #init() {
        this.setItsLoadingArticlesNow(true);
        await this.#loadArticles();
        this.setItsLoadingArticlesNow(false);
        await this.#itsShouldLoadMore();
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

    redirectToLoginPage() {
        const path = "/admin/articles";
        const redirectTo = encodeURIComponent(path);

        window.location.href = "/admin/login?redirect-to=" + redirectTo;
    }

    async #windowsResizeListener() {
        window.addEventListener("resize", () => {
            if (this.#shouldLoadMore() && !this.getItsLoadingArticlesNow() && this.getIsBackendHasMoreContents()) {
                this.#itsShouldLoadMore();
            }
        });
    }

    hideMoveToTrashModalDialog() {
        const modal = document.getElementById('articleMoveToTrashModal');
        const modalInstance = bootstrap.Modal.getOrCreateInstance(modal);
        
        modalInstance.hide();
    }

    showMoveToTrashModalDialog(articleId) {
        const modal = document.getElementById('articleMoveToTrashModal');
        const modalInstance = bootstrap.Modal.getOrCreateInstance(modal);

        this.markArticleForMoveToTrash = articleId;
        
        modalInstance.show();
    }

    cancelMoveArticleToTrash() {
        this.markArticleForMoveToTrash = null;
    }

    async moveArticleToTrash() {
        const tr = document.getElementById("trId-" + this.markArticleForMoveToTrash);
        let res = null;

        try {
            res = await this.viewModel.moveArticleToTrash(this.markArticleForMoveToTrash);
        } catch(error) {
            if (error instanceof UnauthorizedError) {
                this.redirectToLoginPage();
            } else if (error instanceof HttpError) {
                this.alertDanger(error.message);
            } else {
                this.alertDanger(error.message);
            }
        }

        if (res.response === true) {
            this.alertSuccess("The article was moved to trash.");
        } else if (res.response === false) {
            this.alertDanger("Failed to move article to trash.");
        } else {
            this.alertDanger("Failed to move article to trash.");
        }

        tr.remove();
        this.markArticleForMoveToTrash = null;
        this.hideMoveToTrashModalDialog();
    }

    async getArticles(page) {
        let articles = null;
        let res = null;

        try {
            res = await this.viewModel.getArticles(page);
        } catch(error) {
            if (error instanceof UnauthorizedError) {
                main.view.alertClose();
                main.view.alertDanger("Unauthorized.");
                main.view.redirectToLoginPage();
            } else if (error instanceof TypeError) {
                main.view.alertClose();
                main.view.alertDanger("Error : " + error.message);
            } else {
                main.view.alertClose();
                main.view.alertDanger("Error : " + error.message);
            }
        }

        if (res.response === true) {
            if (res.has_more === false) {
                this.setIsBackendHasMoreContents(false);
            }

            for (let i = 0; i < res.data.length; i++) {
                let author = null;

                if (res.data[i].author_last_name !== null) {
                    author = res.data[i].author_first_name + ' ' + res.data[i].author_last_name;
                } else {
                    author = res.data[i].author_first_name;
                }

                const date = new Date(res.data[i].created * 1000);
                const created = date.getDate() + "/" + (date.getMonth()+1) + "/" + date.getFullYear();
                const status = res.data[i].status === 1 ? "Draft" : "Published";
                const article = '<tr id="trId-' + res.data[i].id + '">' +
                                    '<td class="article-title">' +
                                        '<table>' +
                                            '<tr>' +
                                                '<td>' +
                                                    '<a class="link-underline link-underline-opacity-0" style="text-decoration: none;" href="#">' + res.data[i].title + '</a>' +
                                                '</td>' +
                                            '</tr>' +
                                            '<tr class="articles-link-small-tr">' +
                                                '<td class="articles-link-small-td">' +
                                                    '<div class="articles-link-small-div">' +
                                                        '<a target="_blank" href="/admin/article-editor/' + res.data[i].id + '">Edit</a> | <a onClick="main.view.showMoveToTrashModalDialog(\'' + res.data[i].id + '\');" href="#">Move To Trash</a> | <a target="_blank" href="/article/' + res.data[i].id + '">View</a>' +
                                                    '</div>' +
                                                '</td>' +
                                            '</tr>' +
                                        '</table>' +
                                    '</td>' +
                                    '<td>' + author + '</td>' +
                                    '<td>' + res.data[i].visitors + '</td>' +
                                    '<td>' + status + '</td>' +
                                    '<td>' + created + '</td>' +
                                '</tr>';
                
                articles = articles === null ? article : articles + article;
            }
        }

        return articles;
    }

    async #loadArticles() {
        const articles = await this.getArticles(this.#infiniteScrollPage + 1);

        if (articles !== null) {
            const table = '<div id="" class="table-responsive">' +
                                '<table id="tableArticles" class="table table-articles">' +
                                    '<thead>' +
                                        '<tr>' +
                                            '<th>Title</th>' +
                                            '<th>Author</th>' +
                                            '<th>Visitors</th>' +
                                            '<th>Status</th>' +
                                            '<th>Published</th>' +
                                        '</tr>' +
                                    '</thead>' +
                                    '<tbody id="tbodyArticles"></tbody>' +
                                '</table>' +
                            '</div>';
            document.getElementById("cardBody").insertAdjacentHTML("beforeend", table);
            document.getElementById("tbodyArticles").insertAdjacentHTML("beforeend", articles);
            this.#infiniteScrollPage = this.#infiniteScrollPage + 1;
        } else if (articles === null && this.#infiniteScrollPage === 0) {
            const div = '<div class="articles-no-articles-warning text-muted">There are no articles yet.</div>';
            document.getElementById("cardBody").insertAdjacentHTML("beforeend", div);
        }
    }

    async #itsShouldLoadMore() {
        while(this.#shouldLoadMore() && !this.getItsLoadingArticlesNow() && this.getIsBackendHasMoreContents()) {
            this.setItsLoadingArticlesNow(true);
            await this.#loadArticles();
            this.setItsLoadingArticlesNow(false);
        }
    }

    #shouldLoadMore() {
        return document.body.scrollHeight <= window.innerHeight;
    }

    async #infiniteScrollLoad() {
        this.setItsLoadingArticlesNow(true);
        this.addInfiniteScrollSpinners();
        await this.#loadArticles();
        this.setItsLoadingArticlesNow(false);
        this.removeInfiniteScrollSpinners();
    }

    async #infiniteScrollListener() {
        window.addEventListener("scroll", () => {
            if (window.innerHeight + window.scrollY >= document.getElementById("tableArticles").offsetHeight + 250) {
                if (!this.getItsLoadingArticlesNow() && this.getIsBackendHasMoreContents()) {
                    this.#infiniteScrollLoad();
                }
            }
        });
    }

    infiniteScrollSpinner() {
        const spinner = '<div class="d-flex justify-content-center">' +
                            '<div class="spinner-border" role="status">' +
                                '<span class="visually-hidden">Loading...</span>' +
                            '</div>' +
                        '</div>';

        return spinner;
    }

    addInfiniteScrollSpinners() {
        const spinners = '<tr id="spinners">' +
                            '<td class="article-title">' + this.infiniteScrollSpinner() + '</td>' +
                            '<td>' + this.infiniteScrollSpinner() + '</td>' +
                            '<td>' + this.infiniteScrollSpinner() + '</td>' +
                            '<td>' + this.infiniteScrollSpinner() + '</td>' +
                            '<td>' + this.infiniteScrollSpinner() + '</td>' +
                        '</tr>';
        
        document.getElementById("tbodyArticles").insertAdjacentHTML("beforeend", spinners);
    }

    removeInfiniteScrollSpinners() {
        document.getElementById("spinners").remove();
    }

    getItsLoadingArticlesNow() {
        return this.#itsLoadingArticlesNow;
    }

    setItsLoadingArticlesNow(value) {
        this.#itsLoadingArticlesNow = value;
    }

    getIsBackendHasMoreContents() {
        return this.#isBackendHasMoreContents;
    }

    setIsBackendHasMoreContents(value) {
        this.#isBackendHasMoreContents = value;
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

        if (alertElement) {
            const alertInstance = bootstrap.Alert.getOrCreateInstance(alertElement);
            alertInstance.close();
        }
    }
}
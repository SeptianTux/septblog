'use strict';

import { NotFoundError } from "../../admin/js/Errors/NotFoundError.js";
import { Page } from "../../admin/js/Utils/Page.js";
import { CategoryViewModel } from "../ViewModel/CategoryViewModel.js";

export class CategoryView {
    #config = null;
    viewModel = null;

    #pageCounter = 1;
    #itsLoadingArticlesNow = false;
    #isBackendHasMore = true;

    constructor(config) {
        this.#config = config;
        this.viewModel = new CategoryViewModel(this.#config);

        this.init();

        this.#setSiteTitle();
        this.#setNavbarBrand();
        this.#setCopyright();
    }

    async init() {
        this.#setItsLoadingArticlesNow(true);
        this.addInfiniteScrolSpinner();
        await this.loadArticles();
        this.removeInfiniteScrollSpinner();
        this.#setItsLoadingArticlesNow(false);

        this.#infiniteScrollListener();
        this.#itsShouldLoadMore();
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

    async getArticlesFromBackend(categoryName, page) {
        let res = null;

        try {
            res = await this.viewModel.getArticlesFromBackend(categoryName, page);
        } catch(error) {
            if (error instanceof NotFoundError) {
                this.redirectToNotFoundPage();
            } else if (error instanceof TypeError) {
                this.alertDanger("Failed to connect to the network.");
            } else {
                this.alertDanger(error.message);
            }
        }

        for (let i = 0; i < res.data.length; i++) {
            res.data[i].content = res.data[i].content.replace(/&nbsp;/g, " ");
        }

        if (res.has_more === false) {
            this.#isBackendHasMore = false;
        }

        return res;
    }

    getCategoryNameFromUrl() {
        const pathSegments = window.location.pathname.split("/").filter(segment => segment);

        if(pathSegments[1] === null || pathSegments[1] === undefined) {
            return null;
        }

        return pathSegments[1];
    }

    #setItsLoadingArticlesNow(val) {
        this.#itsLoadingArticlesNow = val;
    }

    #shouldLoadMore() {
        return document.body.scrollHeight <= window.innerHeight;
    }

    async #itsShouldLoadMore() {
        while(this.#shouldLoadMore() && this.#isBackendHasMore) {
            this.#setItsLoadingArticlesNow(true);
            this.addInfiniteScrolSpinner();
            await this.loadArticles();
            this.removeInfiniteScrollSpinner();
            this.#setItsLoadingArticlesNow(false);
        }
    }

    async #loadInfiniteScrollListener() {
        this.#setItsLoadingArticlesNow(true);
        await this.loadArticles();
        this.#increaseHomePageCounter();
        this.#setItsLoadingArticlesNow(false);
    }

    async #infiniteScrollListener() {
        window.addEventListener("scroll", () => {
            if (window.innerHeight + window.scrollY >= document.getElementById("categoryArticles").offsetHeight + 80) {
                if (!this.isInfiniteScrollSpinnerExist()) {
                    this.addInfiniteScrolSpinner();
                }

                if (!this.#itsLoadingArticlesNow && this.#isBackendHasMore) {
                    this.#loadInfiniteScrollListener();
                }

                this.removeInfiniteScrollSpinner();
            }
        });
    }

    #increaseHomePageCounter() {
        this.#pageCounter++;
    }

    wordCount(str) {
        const matches = str.match(/[\s\u00A0]/g);

        return matches ? matches.length : 0;
    }

    async #articles() {
        const categoryName = this.getCategoryNameFromUrl();
        let articlesFromBackend = await this.getArticlesFromBackend(categoryName, this.#pageCounter);
        let articles = null;

        this.#increaseHomePageCounter();

        if (articlesFromBackend.data.length !== 0) {
            articles = '';

            for (let i = 0; i < articlesFromBackend.data.length; i++) {
                const date = new Date(articlesFromBackend.data[i].created * 1000);
                const writtenAt = date.getDate() + "/" + (date.getMonth()+1) + "/" + date.getFullYear();
                const articleWordCount = this.wordCount(articlesFromBackend.data[i].content);
                const readMore = articleWordCount < 150 ? '' : '... <a style="text-decoration: none;" href="/article/' +  articlesFromBackend.data[i].id + '">Read more...</a>';
                const authorName = articlesFromBackend.data[i].author.last_name === null ? articlesFromBackend.data[i].author.first_name : articlesFromBackend.data[i].author.first_name + ' ' + articlesFromBackend.data[i].author.last_name;

                articles = articles + '<div class="category-article border-bottom padding-12px">' + 
                                            '<div>' + 
                                                '<h2>' +
                                                    '<a style="text-decoration: none;" href="/article/' + articlesFromBackend.data[i].id + '">' + articlesFromBackend.data[i].title + '</a>' +
                                                '</h2>' +
                                            '</div>' +
                                            '<div class="article-body">' +
                                                '<p>' +
                                                    articlesFromBackend.data[i].content +
                                                    readMore +
                                                '</p>' +
                                            '</div>' +
                                            '<div>' +
                                                '<small>Written by <a style="text-decoration: none;" href="/profile/' + articlesFromBackend.data[i].author.username + '">' + authorName +'</a> in ' + writtenAt + '</small>' +
                                            '</div>' +
                                        '</div>';
            }
        }
        
        return articles;
    }

    async loadArticles() {
        let articles = await this.#articles();

        if (articles === null) {
            if (this.#pageCounter === 2) {
                articles = '<div class="category-articles-no-articles">There are no articles.</div>';
                document.getElementById("categoryArticles").insertAdjacentHTML("beforeend", articles);
            }
        } else {
            document.getElementById("categoryArticles").insertAdjacentHTML("beforeend", articles);
        }
    }

    redirectToNotFoundPage() {
        window.location.href = "/404";
    }

    addInfiniteScrolSpinner() {
        const spinner = '<div id="spinnerCategoryArticlesInfiniteScroll" class="spinner-home-articles-infinite-scroll spinner-border mx-auto d-block " role="status">' +
                            '<span class="visually-hidden">Loading...</span>' +
                        '</div>';

        document.getElementById("categoryArticles").insertAdjacentHTML("beforeend", spinner);
    }

    removeInfiniteScrollSpinner() {
        if (this.isInfiniteScrollSpinnerExist()) {
            document.getElementById("spinnerCategoryArticlesInfiniteScroll").remove();
        }
    }

    isInfiniteScrollSpinnerExist() {
        const spinner = document.getElementById("spinnerCategoryArticlesInfiniteScroll");

        if (spinner == null) {
            return false;
        }

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
'use strict';

import { NotFoundError } from "../../admin/js/Errors/NotFoundError.js";
import { Page } from "../../admin/js/Utils/Page.js";
import { ArticleViewModel } from "../ViewModel/ArticleViewModel.js";

export class ArticleView {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new ArticleViewModel(this.#config);

        this.setArticleData();
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

    

    #getArticleIdFromUrl() {
        const pathSegments = window.location.pathname.split("/").filter(segment => segment);

        if(pathSegments[1] === null || pathSegments[1] === undefined) {
            return null;
        }

        return pathSegments[1];
    }

    async getArticleFromBackend(id) {
        let res = null;

        try {
            res = await this.viewModel.getArticleFromBackend(id);
        } catch(error) {
            if (error instanceof TypeError) {
                this.alertDanger("Failed to connect to the network.");
            } else if (error instanceof NotFoundError) {
                this.redirectToNotFoundPage();
            } else {
                this.alertDanger(error.message);
            }
        }

        if (res.response === true) {
            res.data.content = res.data.content.replace(/&nbsp;/g, " ");

            return res.data;
        } else {
            this.alertDanger("Failed to get article from backend.");
        }

        return null;
    }

    async setArticleData() {
        const articleId = this.#getArticleIdFromUrl();
        const articleData = await this.getArticleFromBackend(articleId);

        console.log(articleData);

        this.setArticleTitle(articleData.title);
        this.setArticleContent(articleData.content);
        this.setArticleCategories(articleData.categories);
        this.setWrittenBy(articleData.author, articleData.created);
    }

    setArticleTitle(title) {
        const tit = '<h2>' + title + '</h2>';
        
        document.getElementById("articleTitle").insertAdjacentHTML("beforeend", tit);
    }

    setArticleContent(articleContent) {
        document.getElementById("articleContent").replaceChildren();
        document.getElementById("articleContent").insertAdjacentHTML("afterbegin", articleContent);
    }

    setArticleCategories(categories) {
        let cat = '<small>';
        
        for (let i = 0; i < categories.length; i++) {
            cat = cat + '<span class="badge text-bg-secondary"><a class="link-light" href="/category/' + categories[i].name + '">' + categories[i].name + '</a></span> ';
        }

        cat = cat + '</small>';

        document.getElementById("articleCategories").insertAdjacentHTML("beforeend", cat);
    }

    setWrittenBy(author, created) {
        const date = new Date(created * 1000);
        const dateCreation = date.getDate() + "/" + date.getMonth() + "/" + date.getFullYear();
        const writerFullName = author.last_name === null ? author.first_name : author.first_name + " " + author.last_name;
        const wB = '<small>Written by <a id="articleWriter" style="text-decoration: none;" href="/user/' + author.username + '">' + writerFullName + '</a> in <a id="articlePublished" style="text-decoration: none;" href="#">' + dateCreation + '</a></small>';

        document.getElementById("writtenBy").insertAdjacentHTML("beforeend", wB);
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
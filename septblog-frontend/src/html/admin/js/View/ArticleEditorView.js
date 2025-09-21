'use strict';

import { ArticleEditorViewModel } from "../ViewModel/ArticleEditorViewModel.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { HttpError } from "../Errors/HttpError.js";
import { Page } from "../Utils/Page.js";
import { User } from "../Utils/User.js";

export class ArticleEditorView {
    viewModel = null;
    quill = null;
    #tagify = null;
    #config = null;
    #articleId = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new ArticleEditorViewModel(this.#config);

        this.#checkCredentials();
        this.#articleEditor();
        this.#tagifyFunc();
        this.ifArticleIdIsSetInTheUrl();
        this.#setLoggedInAs();
        this.#setPageTitle();
        this.#setNavbarBrand();
        this.#setCopyright();

        this.#administratorsNavMenu();
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

    async #checkCredentials() {
        let credentials = null;
        
        try {
            credentials = await this.viewModel.checkCredentials();
        } catch(error) {
            this.alertDanger("Problem in connecting to the server.");
        }

        if (credentials === false) {
            this.#redirectToLoginPage();
        }
    }

    async #tagifyFunc() {
        const inputElem = document.getElementById("articleCategory");
        let whiteList = null;

        try {
            whiteList = await this.viewModel.getArticleCategoriesFromBackend();
        } catch(error) {
            if (error instanceof TypeError) {
                this.alertDanger("Failed to connect to the network.");
            } else if (error instanceof UnauthorizedError) {
                this.#redirectToLoginPage();
            } else if (error instanceof HttpError) {
                this.alertDanger("Unexpected response from backend.");
            } else {
                this.alertDanger("Error. Error message: " + error.message + ".");
            }
        }

        this.#tagify = new Tagify(inputElem, {
            // A list of possible tags. This setting is optional if you want to allow
            // any possible tag to be added without suggesting any to the user.
            whitelist: whiteList
        });
    }

    #getArticleId() {
        return this.#articleId;
    }

    #setArticleId(articleId) {
        this.#articleId = articleId;
    }

    #setArticleIdInTheUrl(articleId) {
        const queryString   = window.location.search;
        window.history.pushState({}, '', '/admin/article-editor/' + articleId);
    }

    #getArticleIdFromUrl() {
        const pathSegments = window.location.pathname.split("/").filter(segment => segment);

        if(pathSegments[1] === null || pathSegments[1] === undefined) {
            return null;
        }

        return pathSegments[2];
    }

    #isArticleIdIsSetInTheUrl() {
        const articleId = this.#getArticleIdFromUrl();

        if (articleId === null || articleId === undefined) {
            return false;
        }

        return true;
    }

    addImageUploadProgressbar() {
        const imageUploadProgressbar = '<div id="removableImageUploadProgressbar" class="upload-progressbar"><div id="uploadProgressbar" class="progress" role="progressbar" aria-label="Example with label" aria-valuenow="0" aria-valuemin="0" aria-valuemax="100"><div id="imageUploadProgressbar" class="progress-bar progress-bar-striped progress-bar-animated" style="width: 0%">0%</div></div></div>';

        document.getElementById("articleEditorContainer").insertAdjacentHTML("beforeend", imageUploadProgressbar);
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

    async #articleEditor() {
        const toolbarOptions = [
            ['bold', 'italic', 'underline', 'strike'],        // toggled buttons
            ['blockquote', 'code-block'],
            ['link', 'image', 'video', 'formula'],

            [{ 'header': 1 }, { 'header': 2 }],               // custom button values
            [{ 'list': 'ordered'}, { 'list': 'bullet' }, { 'list': 'check' }],
            [{ 'script': 'sub'}, { 'script': 'super' }],      // superscript/subscript
            [{ 'indent': '-1'}, { 'indent': '+1' }],          // outdent/indent
            [{ 'direction': 'rtl' }],                         // text direction

            [{ 'size': ['small', false, 'large', 'huge'] }],  // custom dropdown
            [{ 'header': [1, 2, 3, 4, 5, 6, false] }],

            [{ 'color': [] }, { 'background': [] }],          // dropdown with defaults from theme
            [{ 'font': [] }],
            [{ 'align': [] }],

            ['clean']                                         // remove formatting button
        ];

        this.quill = new Quill('#articleEditor', {
            theme: 'snow',
            modules: {
                toolbar: toolbarOptions
            }
        });

        this.quill.getModule('toolbar').addHandler('image', () => {
            const input = document.createElement('input');
            input.setAttribute('type', 'file');
            input.setAttribute('accept', 'image/*');
            input.click();

            input.onchange = async () => {
                let xhr = new XMLHttpRequest();
                const file = input.files[0];

                if (file) {
                    this.addImageUploadProgressbar();
                    
                    const formData = new FormData();
                    
                    formData.append("image", file);

                    try {
                        this.viewModel.articleEditorImageUpload(xhr);
                    } catch(error) {
                        if (error instanceof UnauthorizedError) {
                            this.#redirectToLoginPage();
                        } else {
                            this.alertDanger(error.message);
                        }
                    }

                    xhr.upload.addEventListener('progress', function(e) {
                        if (e.lengthComputable) {
                            let percent = (e.loaded / e.total) * 100;
                            
                            main.view.updateImageUploadProgressbar(percent);
                        }
                    });

                    xhr.addEventListener('load', function() {
                        if (xhr.status === 201) {
                            let responseData = null;

                            try {
                                responseData = JSON.parse(xhr.responseText);
                            } catch (e) {
                                main.view.alertDanger("Failed to parse data.");
                            }

                            const range = main.view.quill.getSelection();
                            
                            main.view.quill.insertEmbed(range.index, 'image', responseData.url);
                            main.view.removeImageUploadProgressbar();
                        } else {
                            main.view.alertDanger("Failed to upload an image. The server returned " + xhr.status + ".");
                        }
                    });

                    xhr.addEventListener('error', function() {
                        main.view.alertDanger("Failed to upload an image.");
                        main.view.removeImageUploadProgressbar();
                    });

                    xhr.send(formData);
                }
            };
        });
    }

    async ifArticleIdIsSetInTheUrl() {
        const isSet = this.#isArticleIdIsSetInTheUrl();

        if (isSet) {
            const articleId = this.#getArticleIdFromUrl();
            let article = null;

            this.#setArticleId(articleId);
            
            try {
                article = await this.viewModel.getArticleFromBackend(articleId);
            } catch(error) {
                this.alertDanger(error.message);
                this.removePublishButton();
                this.removeSaveToDrafthButton();
                this.addSaveButton();
                this.buttonSaveDisabled();
            }

            if (article !== null) {
                this.#articleId = this.#getArticleIdFromUrl();

                this.setArticleTitle(article.data.title);
                this.setSemanticHtmlToArticleEditor(article.data.content);
                this.setArticleCategories(article.data.categories);

                if (article.data.status === 2) {
                    this.removePublishButton();
                    this.removeSaveToDrafthButton();
                    this.addSaveButton();
                }
            }
        }
    }

    getArticleTitle() {
        return document.getElementById("articleTitle").value;
    }

    async setArticleTitle(title) {
        document.getElementById("articleTitle").value = title;
    }

    getSemanticHtmlFromArticleEditor() {
        return this.quill.getSemanticHTML(0, this.quill.getLength());
    }

    async setSemanticHtmlToArticleEditor(htmlContent) {
        this.quill.clipboard.dangerouslyPasteHTML(htmlContent);
    }

    getArticleCategories() {
        let ret = new Array();

        for (let i = 0; i < this.#tagify.value.length; i++) {
            ret[i] = this.#tagify.value[i].value;
        }

        return ret;
    }

    async setArticleCategories(categories) {
        this.#tagify.addTags(categories);
    }

    #validation() {
        if (this.getArticleTitle() === "") {
            this.alertClose();
            this.alertDanger("Article title is empty.");

            return false;
        } else if (this.getSemanticHtmlFromArticleEditor() === "" || this.getSemanticHtmlFromArticleEditor() === "<p></p>") {
            this.alertClose();
            this.alertDanger("Article content is empty.");

            return false;
        }

        return true;
    }

    async saveArticleToDraft() {
        const articleId = this.#getArticleId();
        const articleTitle = this.getArticleTitle();
        const articleContent = this.getSemanticHtmlFromArticleEditor();
        const articleCategories = this.getArticleCategories();
        let res = null;
        const isValid = this.#validation();

        if (isValid) {
            try {
                res = await this.viewModel.saveArticleToDraft(
                    articleId,
                    articleTitle,
                    articleContent,
                    articleCategories
                );
            } catch(error) {
                if (error instanceof UnauthorizedError) {
                    this.#redirectToLoginPage();
                } else {
                    this.alertDanger(error.message);
                }
            }

            if (res.response === true) {
                this.#setArticleId(res.article_id);
                this.#setArticleIdInTheUrl(res.article_id);
                this.alertClose();
                this.alertSuccess("Article is saved to draft.");
            }
        }
    }

    async publishArticle() {
        const articleId = this.#getArticleId();
        const articleTitle = this.getArticleTitle();
        const articleContent = this.getSemanticHtmlFromArticleEditor();
        const articleCategories = this.getArticleCategories();
        let res = null;
        const isValid = this.#validation();

        if (isValid) {
            try {
                res = await this.viewModel.publishArticle(
                    articleId,
                    articleTitle,
                    articleContent,
                    articleCategories
                );
            } catch(error) {
                if (error instanceof UnauthorizedError) {
                    this.#redirectToLoginPage();
                } else {
                    this.alertDanger(error.message);
                }
            }

            if (res.response === true) {
                this.#setArticleId(res.article_id);
                this.#setArticleIdInTheUrl(res.article_id);
                this.removePublishButton();
                this.removeSaveToDrafthButton();
                this.addSaveButton();
                this.alertClose();
                this.alertSuccess("Article is published.");
            }
        }
    }

    async saveArticle() {
        const articleId = this.#getArticleId();
        const articleTitle = this.getArticleTitle();
        const articleContent = this.getSemanticHtmlFromArticleEditor();
        const articleCategories = this.getArticleCategories();
        let res = null;
        const isValid = this.#validation();

        if (isValid) {
            try {
                res = await this.viewModel.saveArticle(
                    articleId,
                    articleTitle,
                    articleContent,
                    articleCategories
                );
            } catch(error) {
                if (error instanceof UnauthorizedError) {
                    this.#redirectToLoginPage();
                } else {
                    this.alertDanger(error.message);
                }
            }

            if (res.response === true) {
                this.#setArticleId(res.article_id);
                this.#setArticleIdInTheUrl(res.article_id);
                this.removePublishButton();
                this.removeSaveToDrafthButton();
                this.alertClose();
                this.alertSuccess("Article is saved.");
            }
        }
    }

    #redirectToLoginPage() {
        let path = "/admin/article-editor";
        
        if(this.#isArticleIdIsSetInTheUrl()) {
            path = path + "/" + this.#getArticleIdFromUrl();
        }

        const redirectTo = encodeURIComponent(path);

        window.location.href = "/admin/login?redirect-to=" + redirectTo;

    }

    #buttonLoading(buttonId) {
        let btn = document.getElementById(buttonId);
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
        let btn = document.getElementById(buttonId);

        if (btn === null) {
            return false;
        }

        btn.replaceChildren();
        btn.textContent = buttonName;
        btn.disabled = false;

        return true;
    }

    buttonPublishLoading() {
        return this.#buttonLoading("publishButton");
    }

    buttonPublishNormal() {
        return this.#buttonNormal("publishButton", "Publish");
    }

    buttonPublishDisabled() {
        return document.getElementById("publishButton").disabled = true;
    }

    buttonPublishEnabled() {
        return document.getElementById("publishButton").disabled = false;
    }

    buttonSaveToDraftLoading() {
        return this.#buttonLoading("saveToDraftButton");
    }

    buttonSaveToDraftNormal() {
        return this.#buttonNormal("saveToDraftButton", "Save To Draft");
    }

    buttonSaveToDraftDisabled() {
        return document.getElementById("saveToDraftButton").disabled = true;
    }

    buttonSaveToDraftEnabled() {
        return document.getElementById("saveToDraftButton").disabled = false;
    }

    buttonSaveLoading() {
        return this.#buttonLoading("saveButton");
    }

    buttonSaveNormal() {
        return this.#buttonNormal("saveButton", "Save");
    }

    buttonSaveDisabled() {
        return document.getElementById("saveButton").disabled = true;
    }

    buttonSaveEnabled() {
        return document.getElementById("saveButton").disabled = false;
    }

    removePublishButton() {
        let button = document.getElementById("publishButton");

        if (button !== null) {
            button.remove();
            return true;
        }

        return false;
    }

    removeSaveToDrafthButton() {
        let button = document.getElementById("saveToDraftButton");

        if (button !== null) {
            button.remove();
            return true;
        }

        return false;
    }

    addSaveButton() {
        const button = '<button id="saveButton" type="button" class="btn btn-primary" onclick="main.view.saveArticle()">Save</button>';

        document.getElementById("buttonContainer").insertAdjacentHTML("beforeend", button);
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
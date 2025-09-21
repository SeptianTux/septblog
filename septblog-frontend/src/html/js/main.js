'use strict';

import { Config } from './config.js';

class Main {
    importedView = null;
    view = null;
    
    constructor() {
        this.fire();
    }

    path() {
        let pathSegments = window.location.pathname.split("/").filter(segment => segment);
        return pathSegments;
    }

    fire() {
        const config = new Config();

        switch(this.path()[0]) {
            case undefined :
                this.importedView = import('./View/IndexView.js');
                this.importedView.then(
                    result => {
                        this.view = new result.IndexView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'article' :
                this.importedView = import('./View/ArticleView.js');
                this.importedView.then(
                    result => {
                        this.view = new result.ArticleView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case '404' :
                this.importedView = import('./View/404View.js');
                this.importedView.then(
                    result => {
                        this.view = new result.NotFoundView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'about' :
                this.importedView = import('./View/AboutView.js');
                this.importedView.then(
                    result => {
                        this.view = new result.AboutView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'contact' :
                this.importedView = import('./View/ContactView.js');
                this.importedView.then(
                    result => {
                        this.view = new result.ContactView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'user' :
                this.importedView = import('./View/UserView.js');
                this.importedView.then(
                    result => {
                        this.view = new result.UserView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'category' :
                this.importedView = import('./View/CategoryView.js');
                this.importedView.then(
                    result => {
                        this.view = new result.CategoryView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'install' :
                switch(this.path()[2]) {
                    case "1" :
                        this.importedView = import('./View/InstallStageOneView.js');
                        this.importedView.then(
                            result => {
                                this.view = new result.InstallStageOneView(config);
                            },
                            error => {
                                console.log(error);
                            }
                        );
                        break;
                    case "2" :
                        this.importedView = import('./View/InstallStageTwoView.js');
                        this.importedView.then(
                            result => {
                                this.view = new result.InstallStageTwoView(config);
                            },
                            error => {
                                console.log(error);
                            }
                        );
                        break;
                    default :
                        break;
                }
                break;
        }
    }
}

window.main = new Main();
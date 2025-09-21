'use strict';

import { Config } from "../../../js/config.js";

class Main {
    importedView = null;
    view = null;
    
    constructor() {
        this.sidebarToggle();
        this.fire();
    }

    sidebarToggle() {
        window.addEventListener('DOMContentLoaded', event => {

            // Toggle the side navigation
            const sidebarToggle = document.body.querySelector('#sidebarToggle');
            if (sidebarToggle) {
                // Uncomment Below to persist sidebar toggle between refreshes
                // if (localStorage.getItem('sb|sidebar-toggle') === 'true') {
                //     document.body.classList.toggle('sb-sidenav-toggled');
                // }
                sidebarToggle.addEventListener('click', event => {
                    event.preventDefault();
                    document.body.classList.toggle('sb-sidenav-toggled');
                    localStorage.setItem('sb|sidebar-toggle', document.body.classList.contains('sb-sidenav-toggled'));
                });
            }

        });
    }

    path() {
        let pathSegments = window.location.pathname.split("/").filter(segment => segment);
        return pathSegments;
    }

    fire() {
        const config = new Config();

        switch(this.path()[1]) {
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
            case 'login' :
                this.importedView = import('./View/LoginView.js');

                this.importedView.then(
                    result => {
                        this.view = new result.LoginView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'logout' :
                this.importedView = import('./View/LogoutView.js');

                this.importedView.then(
                    result => {
                        this.view = new result.LogoutView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'article-editor' :
                this.importedView = import('./View/ArticleEditorView.js');

                this.importedView.then(
                    result => {
                        this.view = new result.ArticleEditorView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'articles' :
                this.importedView = import('./View/ArticlesView.js');

                this.importedView.then(
                    result => {
                        this.view = new result.ArticlesView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'trashed-articles' :
                this.importedView = import('./View/TrashedArticlesView.js');

                this.importedView.then(
                    result => {
                        this.view = new result.TrashedArticlesView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'security' : {
                switch(this.path()[2]) {
                    case 'change-email-address' :
                        this.importedView = import('./View/ChangeEmailAddressView.js');

                        this.importedView.then(
                            result => {
                                this.view = new result.ChangeEmailAddressView(config);
                            },
                            error => {
                                console.log(error);
                            }
                        );
                        break;
                    case 'change-password' :
                        this.importedView = import('./View/ChangePasswordView.js');

                        this.importedView.then(
                            result => {
                                this.view = new result.ChangePasswordView(config);
                            },
                            error => {
                                console.log(error);
                            }
                        );
                        break;
                    default : 
                        this.importedView = import('./View/SecurityView.js');

                        this.importedView.then(
                            result => {
                                this.view = new result.SecurityView(config);
                            },
                            error => {
                                console.log(error);
                            }
                        );
                        break;
                }
                break;
            }
            case 'forgot-password' :
                this.importedView = import('./View/ForgotPasswordView.js');

                this.importedView.then(
                    result => {
                        this.view = new result.ForgotPasswordView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'privacy-policy' :
                this.importedView = import('./View/PrivacyPolicyView.js');

                this.importedView.then(
                    result => {
                        this.view = new result.PrivacyPolicyView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'profile' :
                this.importedView = import('./View/ProfileView.js');

                this.importedView.then(
                    result => {
                        this.view = new result.ProfileView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'settings' :
                this.importedView = import('./View/SettingsView.js');

                this.importedView.then(
                    result => {
                        this.view = new result.SettingsView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'signup' :
                this.importedView = import('./View/SignupView.js');

                this.importedView.then(
                    result => {
                        this.view = new result.SignupView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'suspended-user' :
                this.importedView = import('./View/SuspendedUserView.js');

                this.importedView.then(
                    result => {
                        this.view = new result.SuspendedUserView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'terms-and-conditions' :
                this.importedView = import('./View/TermsAndConditionsView.js');

                this.importedView.then(
                    result => {
                        this.view = new result.TermsAndConditionsView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'users' :
                this.importedView = import('./View/UsersView.js');

                this.importedView.then(
                    result => {
                        this.view = new result.UsersView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
            case 'visitors' :
                this.importedView = import('./View/VisitorsView.js');

                this.importedView.then(
                    result => {
                        this.view = new result.VisitorsView(config);
                    },
                    error => {
                        console.log(error);
                    }
                );
                break;
        }
    }
}

window.main = new Main();
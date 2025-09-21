'use strict';

import { Config } from "../../../js/config.js";
import { HttpError } from "../Errors/HttpError.js";
import { InternalServerError } from "../Errors/InternalServerError.js";
import { UnauthorizedError } from "../Errors/UnauthorizedError.js";

export class User {
    static async checkCredentials() {
        const accessToken = localStorage.getItem("access_token");
        let config = new Config();

        if (!accessToken) {
            return false;
        }

        try {
            const url = config.backendProtocol + config.backendHost + ":" + config.backendPort + "/admin/check-credentials";
            const checkCredentials = await fetch(url, {
                method: "GET",
                headers: {
                    'Authorization': `Bearer ${accessToken}`
                }
            });

            let checkCredentialsJson = await checkCredentials.json();

            if(checkCredentialsJson.response === true && checkCredentialsJson.have_credentials === true) {
                return true;
            } else {
                return false;
            }
        } catch(error) {
            throw error;
        }
    }

    static async getLoggedInAs() {
        const accessToken = localStorage.getItem("access_token");
        let get = null;
        let config = new Config();

        if (!accessToken) {
            throw new UnauthorizedError("Invalid credentials.");
        }

        try {
            const url = config.backendProtocol + config.backendHost + ":" + config.backendPort + "/admin/logged-in-as";

            get = await fetch(url, {
                method: "GET",
                headers: {
                    'Authorization': `Bearer ${accessToken}`
                }
            });
        } catch(error) {
            throw error;
        }

        if (get.status === 200) {
            const resJson = await get.json();

            return resJson.data.full_name;
        } else if (get.status === 500) {
            const resJson = await get.json();

            throw new InternalServerError(resJson.error_message);
        } else if (get.status === 401) {
            throw new UnauthorizedError("Unauthorized.");
        } else {
            throw new HttpError("HTTP error. " + get.status + ".");
        }
    }

    static async getUserLevel() {
        const accessToken = localStorage.getItem("access_token");
        let res = null;
        let config = new Config();

        if (!accessToken) {
            throw new UnauthorizedError("Invalid credentials.");
        }

        try {
            const url = config.backendProtocol + config.backendHost + ":" + config.backendPort + "/admin/get-user-level";
            res = await fetch(url, {
                method: "GET",
                headers: {
                    'Authorization': `Bearer ${accessToken}`
                }
            });
        } catch(error) {
            throw error;
        }

        if (res.status === 200) {
            const resJson = await res.json();

            return resJson.data.user_level;
        } else if (res.status === 401) {
            throw new UnauthorizedError("Unauthorized.");
        } else if (res.status === 500) {
            const resJson = await res.json();

            throw new InternalServerError(resJson.error_message);
        } else {
            throw new HttpError("HTTP error. " + res.status + ".");
        }
    }

    static async administratorsNavMenu() {
        let userLevel = null;

        try {
            userLevel = await this.getUserLevel();
        } catch(error) {
            throw error;
        }

        if (userLevel === 'administrator') {
            const menuUsers = '<a class="nav-link" href="/admin/users">' +
                                    '<div class="sb-nav-link-icon"><i class="fa-solid fa-users"></i></div>' +
                                    'Users' +
                                '</a>';
            const menuSettings = '<a class="nav-link" href="/admin/settings">' +
                                        '<div class="sb-nav-link-icon"><i class="fas fa-cogs"></i></div>' +
                                        'Settings' +
                                    '</a>';
            
            document.getElementById("administratorsNavMenuUsers").insertAdjacentHTML("beforeend", menuUsers);
            document.getElementById("administratorsNavMenuSettings").insertAdjacentHTML("beforeend", menuSettings);
        }
    }
}
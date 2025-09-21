'use strict';

import { Config } from "../../../js/config.js";
import { HttpError } from "../Errors/HttpError.js";
import { InternalServerError } from "../Errors/InternalServerError.js";
import { User } from "./User.js";

export class Page {
    static async getSiteTitle() {
        let get = null;
        let config = new Config();

        try {
            const url = config.backendProtocol + config.backendHost + ":" + config.backendPort + "/get-site-title";

            get = await fetch(url, {
                method: "GET"
            });
        } catch(error) {
            throw error;
        }

        if (get.status === 200) {
            const resJson = await get.json();

            return resJson.data.site_title;
        } else if (get.status === 500) {
            const resJson = await get.json();

            throw new InternalServerError(resJson.error_message);
        } else {
            throw new HttpError("HTTP error. " + get.status + ".");
        }
    }

    static async getSiteTagline() {
        let get = null;
        let config = new Config();

        try {
            const url = config.backendProtocol + config.backendHost + ":" + config.backendPort + "/get-site-tagline";

            get = await fetch(url, {
                method: "GET"
            });
        } catch(error) {
            throw error;
        }

        if (get.status === 200) {
            const resJson = await get.json();

            return resJson.data.site_tagline;
        } else if (get.status === 500) {
            const resJson = await get.json();

            throw new InternalServerError(resJson.error_message);
        } else {
            throw new HttpError("HTTP error. " + get.status + ".");
        }
    }

    static async getFullYear() {
        let get = null;
        let config = new Config();

        try {
            const url = config.backendProtocol + config.backendHost + ":" + config.backendPort + "/get-full-year";

            get = await fetch(url, {
                method: "GET"
            });
        } catch(error) {
            throw error;
        }

        const resJson = await get.json();

        return resJson.data.full_year;
    }

    static async setSiteTitle() {
        const siteTitle = await this.getSiteTitle();
        const siteTagline = await this.getSiteTagline();

        document.title = siteTitle + " | " + siteTagline;
    }

    static async setSiteTitleAdmin() {
        const siteTitle = await this.getSiteTitle();

        document.title = siteTitle + " | Admin";
    }

    static async setNavbarBrand() {
        const siteTitle = await this.getSiteTitle();
        const navbarBrand = document.getElementById("navbarBrand");

        navbarBrand.textContent = siteTitle;
    }

    static async setCopyright() {
        const fullYear = await this.getFullYear();
        const copyrightText = "Copyright &copy; " + fullYear;
        const copyrightElement = document.getElementById("copyright");

        copyrightElement.replaceChildren();
        copyrightElement.insertAdjacentHTML("beforeend", copyrightText);
    }

    static async setLoggedInAs() {
        const user = await User.getLoggedInAs();

        if (user !== null) {
            document.getElementById("loggedInAs").replaceChildren();
            document.getElementById("loggedInAs").insertAdjacentHTML("beforeend", user);
        }
    }
}
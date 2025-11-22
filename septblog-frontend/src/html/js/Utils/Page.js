'use strict';

import { Config } from "../config.js";
import { HttpError } from "../../admin/js/Errors/HttpError.js";
import { InternalServerError } from "../../admin/js/Errors/InternalServerError.js";
//import { User } from "./User.js";

export class Page {
    static async getSiteTitle() {
        let get = null;
        let config = new Config();

        try {
            const url = config.backendUrl + "/get-site-title";

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
            const url = config.backendUrl + "/get-site-tagline";

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
            const url = config.backendUrl + "/get-full-year";

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

    static async setNavbarBrand() {
        const siteTitle = await this.getSiteTitle();
        const navbarBrand = document.getElementById("navbarBrand");

        navbarBrand.textContent = siteTitle;
    }

    static async setCopyright() {
        const fullYear = await this.getFullYear();
        const copyrightText = "Copyright &copy; " + fullYear;
        const copyrightElement = document.getElementById("copyright");

        if (copyrightElement) {
            copyrightElement.replaceChildren();
            copyrightElement.insertAdjacentHTML("beforeend", copyrightText);
        }
    }
}
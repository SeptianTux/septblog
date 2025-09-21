'use strict';

import { LoginModel } from "../Model/LoginModel.js";
import { User } from "../Utils/User.js";

export class LoginViewModel {
    model = null;
    config = null;

    constructor(config) {
        this.config = config;
        this.model = new LoginModel(this.config);
    }

    async checkCredentials() {
        let check = null;

        try {
            check = await User.checkCredentials();
        } catch(error) {
            throw error;
        }

        return check;
    }

    async login(email, password) {
        let loginModel = null;
        
        try {
            loginModel = await this.model.login(email, password);
        } catch(error) {
            throw error;
        }

        if (loginModel.status === 200) {
            const login = await loginModel.json();

            return login;
        } else {
            return false;
        }
    }
}
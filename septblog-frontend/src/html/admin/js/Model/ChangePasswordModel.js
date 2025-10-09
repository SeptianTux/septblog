'use strict';

export class ChangePasswordModel {
    #config = null;

    constructor(config) {
        this.#config = config;
    }

    async putPasswordToBackend(oldPassword, newPassword, newPasswordRepeat) {
        const accessToken = localStorage.getItem("access_token");
        let res = null;

        if (!accessToken) {
            throw new UnauthorizedError("Invalid credentials.");
        }

        const body = JSON.stringify({
            old_password: oldPassword,
            new_password: newPassword,
            new_password_repeat: newPasswordRepeat
        });

        const url = this.#config.backendUrl + "/admin/security/change-password"; 

        try {
            res = await fetch(url, {
                method: "PUT",
                credentials: "include",
                headers: {
                    "Content-Type": "application/json",
                    'Authorization': `Bearer ${accessToken}`
                },
                body: body
            });
        } catch(error) {
            throw error;
        }

        return res;
    }
}
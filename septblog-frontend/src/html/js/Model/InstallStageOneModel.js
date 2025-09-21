'use strict';

export class InstallStageOneModel {
    #config = null;

    constructor(config) {
        this.#config = config;
    }

    async alreadyInstalled() {
        let get = null;

        try {
            const url = this.#config.backendProtocol + this.#config.backendHost + ":" + this.#config.backendPort + "/already-installed";
            
            get = await fetch(url, {
                method: "GET"
            });
        } catch(error) {
            throw error;
        }

        return get;
    }
}
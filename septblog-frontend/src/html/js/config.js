'use strict';

export class Config {
    backendProtocol  = null;
    backendHost      = null;
    backendPort      = 0;

    constructor() {
        this.backendProtocol = "http://";
        this.backendHost = "127.0.0.1";
        this.backendPort = 8080;
    }
}
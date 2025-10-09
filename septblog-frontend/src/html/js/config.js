'use strict';

class Backend {
    protocol    = "protocol";       // Protocol
    host        = "host";           // The host
    path        = null;             // This is optional. Set it to null if you dont use path.
    port        = 0;                // Port
}

export class Config {
    backend     = null;
    backendUrl  = null;

    constructor() {
        this.backend = new Backend();
        this.backendUrl = this.backend.path === null
                            ?
                                this.backend.protocol + this.backend.host + ":" + this.backend.port
                            :
                                this.backend.protocol + this.backend.host + ":" + this.backend.port + "/" + this.backend.path;
    }
}

/*
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
*/
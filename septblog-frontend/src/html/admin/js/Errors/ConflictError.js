'use strict';

export class ConflictError extends Error {
    constructor(message, response) {
        super(message);
        this.name = "ConflictError";
        this.response = response;
    }
}
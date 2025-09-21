'use strict';

export class BadRequestError extends Error {
    constructor(message, response) {
        super(message);
        this.name = "BadRequestError";
        this.response = response;
    }
}
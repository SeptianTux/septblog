'use strict';

export class NotFoundError extends Error {
    constructor(message, response) {
        super(message);
        this.name = "NotFoundError";
    }
}
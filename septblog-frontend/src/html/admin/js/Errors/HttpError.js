'use strict';

export class HttpError extends Error {
  constructor(message) {
    super(message);
    this.name = "HttpError";
  }
}
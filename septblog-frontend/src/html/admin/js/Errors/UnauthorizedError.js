'use strict';

export class UnauthorizedError extends Error {
  constructor(message) {
    super(message); // (1)
    this.name = "UnauthorizedError"; // (2)
  }
}
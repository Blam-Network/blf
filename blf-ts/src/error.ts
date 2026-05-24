export class BlfError extends Error {
    constructor(message: string) {
      super(message);
      this.name = "BlfError";
    }
  }
  
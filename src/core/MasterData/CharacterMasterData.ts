import { readdirSync, readFileSync } from "fs";
import { join } from "path";

export type CharacterSheet = {
  id: string;
  displayName: string;
  price: number;
};
export class CharacterMasterData {
  private _characters: CharacterSheet[];

  constructor(pattern: string) {
    const items = readdirSync(pattern).map((e) => join(pattern, e));
    const characterSheets = items.filter((e) => {
      const regex = /.*\.json$/;
      return regex.test(e);
    });

    this._characters = characterSheets.map((e) => {
      return JSON.parse(readFileSync(e, { encoding: "utf-8" }));
    });
  }

  getPurchaseData() {
    return this._characters.map((e) => ({
      displayName: e.displayName,
      price: e.price,
    }));
  }
}

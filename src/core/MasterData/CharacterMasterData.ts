import { readdirSync, readFileSync } from "fs";
import { join } from "path";
import {
  CharacterSheet,
  ICharacterSheets,
} from "../Services/PurchaseDataService.js";

export class CharacterMasterData implements ICharacterSheets {
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

  getAll(): CharacterSheet[] {
    return [...this._characters];
  }
}

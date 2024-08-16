import { CharacterPurchaseBaseDataFactory } from "./CharacterPurchaseBaseDataFactory.js";

export type CharacterPurchaseBaseData = {
  id: string;
  callId: number;
  displayName: string;
  price: number;
};

export type CallIdTableItem = {
  callId: number;
  objectId: string;
};

export interface ICallIdTable {
  get(): CallIdTableItem[];
}

export type CharacterSheet = {
  id: string;
  displayName: string;
  price: number;
};

export interface ICharacterSheets {
  getAll(): CharacterSheet[];
}

export class PurchaseDataService {
  constructor(
    private readonly _characterSheets: ICharacterSheets,
    private readonly _purchaseCallIdTable: ICallIdTable,
    private readonly _characterPurchaseBaseDataFactory: CharacterPurchaseBaseDataFactory,
  ) {}

  getAllCharacterBaseData(): CharacterPurchaseBaseData[] {
    const characterData = this._characterSheets.getAll();

    const callIdTable = this._purchaseCallIdTable.get();

    return this._characterPurchaseBaseDataFactory.create(
      characterData,
      callIdTable,
    );
  }
}

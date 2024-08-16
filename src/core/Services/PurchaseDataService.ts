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

export class PurchaseDataService {
  constructor(
    private readonly _purchaseCallIdTable: ICallIdTable,
    private readonly _characterPurchaseBaseDataFactory: CharacterPurchaseBaseDataFactory,
  ) {}

  getAllCharacterBaseData(): CharacterPurchaseBaseData[] {
    const characterData = [
      {
        id: "test:character:demo-ko",
        displayName: "テスト子",
        price: 2000,
      },
    ];

    const callIdTable = this._purchaseCallIdTable.get();

    return this._characterPurchaseBaseDataFactory.create(
      characterData,
      callIdTable,
    );
  }
}

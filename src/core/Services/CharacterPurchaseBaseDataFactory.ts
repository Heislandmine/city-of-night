import { CharacterSheet } from "../MasterData/CharacterMasterData.js";
import {
  CallIdTableItem,
  CharacterPurchaseBaseData,
} from "./PurchaseDataService.js";

export class CharacterSheetDataNotFound extends Error {}

export class CharacterPurchaseBaseDataFactory {
  create(
    characterSheets: CharacterSheet[],
    callIdTable: CallIdTableItem[],
  ): CharacterPurchaseBaseData[] {
    const result: CharacterPurchaseBaseData[] = [];

    for (const tableItem of callIdTable) {
      const character = characterSheets.find(
        (e) => e.id === tableItem.objectId,
      );

      if (!character) {
        throw new CharacterSheetDataNotFound(
          `id:${tableItem.objectId}を持つキャラクターシートが見つかりません`,
        );
      }

      result.push({
        id: character.id,
        callId: tableItem.callId,
        displayName: character.displayName,
        price: character.price,
      });
    }
    return result;
  }
}

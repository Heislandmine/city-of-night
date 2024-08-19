import { UserInventory } from "../AppState/UserInventory.js";

export class PurchaseService {
  constructor(private inventory: UserInventory) {}

  purchaseCharacter(targetCharacterId: string, price: number) {
    return this.inventory.addCharacterByPurchase(targetCharacterId, price);
  }
}

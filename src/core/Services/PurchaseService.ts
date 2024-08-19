export class PurchaseService {
  constructor(
    private inventory: { ownedCharacter: string[]; moneyAmount: number },
  ) {}

  purchaseCharacter(targetCharacterId: string, price: number) {
    if (this.inventory.moneyAmount - price < 0) {
      return false;
    }

    this.inventory.moneyAmount -= price;
    this.inventory.ownedCharacter.push(targetCharacterId);

    return true;
  }
}

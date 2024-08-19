export class UserInventory {
  private _ownedCharacter: string[];
  private _moneyAmount: number;

  constructor(moneyAmount: number) {
    this._ownedCharacter = [];
    this._moneyAmount = moneyAmount;
  }

  addCharacterByPurchase(id: string, price: number): boolean {
    if (this.moneyAmount - price < 0) {
      return false;
    }

    this._moneyAmount -= price;
    this._ownedCharacter.push(id);

    return true;
  }

  get ownedCharacter() {
    return [...this._ownedCharacter];
  }

  get moneyAmount() {
    return this._moneyAmount;
  }
}

import React, { useState } from "react";
import { render } from "ink";
import { GameProgressService } from "../core/Services/GameProgressService.js";
import { TopMenuView } from "./component/TopMenuView.js";
import { PurchaseAvailableCharacterListView } from "./component/PurchaseAvailableCharacterListView.js";
import { PurchaseDataService } from "../core/Services/PurchaseDataService.js";
import { PurchaseService } from "../core/Services/PurchaseService.js";

const App = ({
  gameProgressService,
  purchaseDataService,
  purchaseService,
}: {
  gameProgressService: GameProgressService;
  purchaseDataService: PurchaseDataService;
  purchaseService: PurchaseService;
}) => {
  const [currentViewName, setCurrentViewName] = useState("Top");
  const navigateView = (currentViewName: string) => {
    switch (currentViewName) {
      case "Top":
        return (
          <TopMenuView
            gameProgressService={gameProgressService}
            navigate={setCurrentViewName}
          />
        );
      case "purchaseCharacter":
        return (
          <PurchaseAvailableCharacterListView
            gameProgressService={gameProgressService}
            purchaseDataService={purchaseDataService}
            purchaseService={purchaseService}
            navigate={setCurrentViewName}
          />
        );
    }
  };

  return navigateView(currentViewName);
};

export const startUI = (
  gameProgressService: GameProgressService,
  purchaseDataService: PurchaseDataService,
  purchaseService: PurchaseService,
) => {
  render(
    <App
      gameProgressService={gameProgressService}
      purchaseDataService={purchaseDataService}
      purchaseService={purchaseService}
    ></App>,
  );
};

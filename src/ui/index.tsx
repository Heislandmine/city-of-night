import React, { useState } from "react";
import { render } from "ink";
import { GameProgressService } from "../core/Services/GameProgressService.js";
import { TopMenuView } from "./component/TopMenuView.js";
import { PurchaseAvailableCharacterListView } from "./component/PurchaseAvailableCharacterListView.js";

const App = ({
  gameProgressService,
}: {
  gameProgressService: GameProgressService;
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
            navigate={setCurrentViewName}
          />
        );
    }
  };

  return navigateView(currentViewName);
};

export const startUI = (gameProgressService: GameProgressService) => {
  render(<App gameProgressService={gameProgressService}></App>);
};

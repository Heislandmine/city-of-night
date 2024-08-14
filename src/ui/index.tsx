import React, { useEffect, useState } from "react";
import { Box, render, Spacer, Text, useInput } from "ink";
import { GameProgressService } from "../core/Services/GameProgressService.js";
import { TopMenuView } from "./component/TopMenuView.js";
import { TopActionMenu } from "./component/TopActionMenu.js";
import { TopStatusBar } from "./component/TopStatusBar.js";

const PurchaseAvailableCharacterListView = ({
  gameProgressService,
  navigate,
}: {
  gameProgressService: GameProgressService;
  navigate: (viewName: string) => void;
}) => {
  const [userInput, setUserInput] = useState("");
  const [outputString, setOutputString] = useState("");
  const progress = gameProgressService.currentProgress();

  useEffect(() => {
    setOutputString(userInput);
  }, [userInput]);

  useInput((input, key) => {
    if (input) {
      setUserInput((prev) => prev + input);
    }

    if (key.return) {
      switch (userInput) {
        case "200":
          navigate("Top");
          break;

        default:
          setOutputString("不正な値です");
      }
      setUserInput("");
    }
  });

  return (
    <Box width={"100%"} flexDirection="column">
      <TopStatusBar
        date={progress.date}
        phase={progress.phase}
        daysPassed={progress.daysPassed}
        leftDays={progress.lefDays}
      />
      <TopActionMenu
        menuItems={[
          { callNumber: 1, displayName: "デモ子" },
          { callNumber: 200, displayName: "戻る" },
        ]}
      />
      <Text>{outputString}</Text>
    </Box>
  );
};

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

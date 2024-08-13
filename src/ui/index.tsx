import React, { useEffect, useState } from "react";
import { Box, render, Spacer, Text, useInput } from "ink";
import { TopActionMenu } from "./component/TopActionMenu.js";
import { TopStatusBar } from "./component/TopStatusBar.js";
import { GameProgressService } from "../core/Services/GameProgressService.js";

const PurchaseAvailableCharacterListView = () => {
  return <Text>ここに購入可能なキャラクターが表示されます</Text>;
};

const navigateView = (currentViewName: string) => {
  switch (currentViewName) {
    case "Top":
      return (
        <TopActionMenu
          menuItems={[
            { callNumber: 100, displayName: "調教" },
            { callNumber: 300, displayName: "奴隷購入" },
            { callNumber: 200, displayName: "保存" },
          ]}
        />
      );
    case "purchaseCharacter":
      return <PurchaseAvailableCharacterListView />;
  }
};

const topActionMenuItems = [
  { callNumber: 100, displayName: "調教" },
  { callNumber: 300, displayName: "奴隷購入" },
  { callNumber: 200, displayName: "保存" },
];

const App = ({
  gameProgressService,
}: {
  gameProgressService: GameProgressService;
}) => {
  const [currentViewName, setCurrentViewName] = useState("Top");
  const progress = gameProgressService.currentProgress();
  const [userInput, setUserInput] = useState("");
  const [outputString, setOutputString] = useState("");

  useEffect(() => {
    setOutputString(userInput);
  }, [userInput]);

  useInput((input, key) => {
    if (input) {
      setUserInput((prev) => prev + input);
    }

    if (key.return) {
      switch (userInput) {
        case "300":
          setCurrentViewName("purchaseCharacter");
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
      {navigateView(currentViewName)}
      <Text>{outputString}</Text>
    </Box>
  );
};

export const startUI = (gameProgressService: GameProgressService) => {
  render(<App gameProgressService={gameProgressService}></App>);
};

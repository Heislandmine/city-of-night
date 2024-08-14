import React, { useEffect, useState } from "react";
import { GameProgressService } from "../../core/Services/GameProgressService.js";
import { Box, Text, useInput } from "ink";
import { TopStatusBar } from "./TopStatusBar.js";
import { TopActionMenu } from "./TopActionMenu.js";
import { FooterMenu } from "./FooterMenu.js";

export const PurchaseAvailableCharacterListViewActionMenu = ({
  menuItems,
}: {
  menuItems: { callNumber: number; displayName: string; price: number }[];
}) => {
  return (
    <Box width={"100%"} borderStyle={"classic"} gap={2}>
      {menuItems.map((e) => (
        <Text key={e.displayName}>
          <Text>{`[${e.callNumber}]${e.displayName}`}</Text>
          <Text> </Text>
          <Text bold>{`${e.price}G`}</Text>
        </Text>
      ))}
    </Box>
  );
};

export const PurchaseAvailableCharacterListView = ({
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
        case "999":
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
      <PurchaseAvailableCharacterListViewActionMenu
        menuItems={[
          { callNumber: 1, displayName: "デモ子", price: 2000 },
          { callNumber: 2, displayName: "デモ子2", price: 3000 },
        ]}
      />
      <FooterMenu menuItems={[{ callNumber: 999, displayName: "戻る" }]} />
      <Text>{outputString}</Text>
    </Box>
  );
};

import React from "react";
import { Box, render, Spacer, Text } from "ink";

const TopStatusBar = () => {
  return (
    <Box borderStyle={"single"} width={"100%"}>
      <Text>2124/4/1</Text>
      <Spacer />
      <Text>(昼)</Text>
      <Spacer />
      <Text>10日目</Text>
      <Spacer />
      <Text>残り200日</Text>
    </Box>
  );
};

const TopActionMenu = ({
  menuItems,
}: {
  menuItems: { callNumber: number; displayName: string }[];
}) => {
  return (
    <Box width={"100%"} borderStyle={"classic"} gap={2}>
      {menuItems.map((e) => (
        <Text key={e.displayName}>{`[${e.callNumber}]${e.displayName}`}</Text>
      ))}
    </Box>
  );
};
const App = () => {
  return (
    <Box width={"100%"} flexDirection="column">
      <TopStatusBar></TopStatusBar>
      <TopActionMenu
        menuItems={[
          { callNumber: 100, displayName: "調教" },
          { callNumber: 200, displayName: "保存" },
        ]}
      />
    </Box>
  );
};

export const startUI = () => {
  render(<App></App>);
};

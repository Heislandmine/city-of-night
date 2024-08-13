import React from "react";
import { Box, render, Spacer, Text } from "ink";
import { TopActionMenu } from "./component/TopActionMenu.js";
import { TopStatusBar } from "./component/TopStatusBar.js";

const App = () => {
  return (
    <Box width={"100%"} flexDirection="column">
      <TopStatusBar />
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

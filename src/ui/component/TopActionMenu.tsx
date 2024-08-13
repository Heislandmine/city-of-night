import React from "react";
import { Box, Text } from "ink";

export const TopActionMenu = ({
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

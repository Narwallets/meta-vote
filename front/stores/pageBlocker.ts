import create from "zustand/vanilla";
import type { PageBlockerState } from "../components";

export const blockerStore = create<PageBlockerState>(() => ({
  message: "Confirm this action in your wallet",
  isActive: false,
}));

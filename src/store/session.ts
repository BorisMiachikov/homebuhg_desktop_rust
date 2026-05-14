import { create } from "zustand";
import { api } from "../lib/api";

interface SessionState {
  userId: string | null;
  householdId: string | null;
  ready: boolean;
  bootstrap: () => Promise<void>;
}

export const useSession = create<SessionState>((set, get) => ({
  userId: null,
  householdId: null,
  ready: false,
  bootstrap: async () => {
    if (get().ready) return;
    const info = await api.session.bootstrap();
    set({ userId: info.userId, householdId: info.householdId, ready: true });
  },
}));

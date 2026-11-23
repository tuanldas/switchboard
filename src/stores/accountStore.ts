import { create } from "zustand";
import {
  AccountInfo,
  listAccounts,
  addAccount,
  removeAccount,
  switchAccount,
  getActiveAccount,
} from "../lib/tauri";

export interface AccountState {
  accounts: AccountInfo[];
  activeAccount: AccountInfo | null;
  loading: boolean;
  error: string | null;
  fetchAccounts: () => Promise<void>;
  addNewAccount: (provider: string, name: string, label?: string) => Promise<void>;
  deleteAccount: (id: string) => Promise<void>;
  switchTo: (id: string) => Promise<void>;
}

export const useAccountStore = create<AccountState>((set, get) => ({
  accounts: [],
  activeAccount: null,
  loading: false,
  error: null,

  fetchAccounts: async () => {
    set({ loading: true, error: null });
    try {
      const [accounts, activeAccount] = await Promise.all([
        listAccounts(),
        getActiveAccount(),
      ]);
      set({ accounts, activeAccount: activeAccount ?? null, loading: false });
    } catch (err) {
      set({
        error: err instanceof Error ? err.message : String(err),
        loading: false,
      });
    }
  },

  addNewAccount: async (provider: string, name: string, label?: string) => {
    set({ loading: true, error: null });
    try {
      await addAccount(provider, name, label);
      await get().fetchAccounts();
      set({ loading: false });
    } catch (err) {
      set({
        error: err instanceof Error ? err.message : String(err),
        loading: false,
      });
    }
  },

  deleteAccount: async (id: string) => {
    set({ loading: true, error: null });
    try {
      await removeAccount(id);
      await get().fetchAccounts();
      set({ loading: false });
    } catch (err) {
      set({
        error: err instanceof Error ? err.message : String(err),
        loading: false,
      });
    }
  },

  switchTo: async (id: string) => {
    set({ loading: true, error: null });
    try {
      await switchAccount(id);
      await get().fetchAccounts();
      set({ loading: false });
    } catch (err) {
      set({
        error: err instanceof Error ? err.message : String(err),
        loading: false,
      });
    }
  },
}));

export const accountStore = useAccountStore;
export default useAccountStore;

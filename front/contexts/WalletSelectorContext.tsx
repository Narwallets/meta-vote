import React, { useCallback, useContext, useEffect, useState } from "react";
import { map, distinctUntilChanged } from "rxjs";
import {
  NetworkId,
  setupWalletSelector,
  Wallet,
} from "@near-wallet-selector/core";
import type { WalletSelector, AccountState } from "@near-wallet-selector/core";
import { setupModal } from "@near-wallet-selector/modal-ui";
import type { WalletSelectorModal } from "@near-wallet-selector/modal-ui";
import { setupMyNearWallet } from "@near-wallet-selector/my-near-wallet";
import { setupMathWallet } from "@near-wallet-selector/math-wallet";
import { setupNarwallets } from "@near-wallet-selector/narwallets"
import { setupNightly } from "@near-wallet-selector/nightly"
import { setupLedger } from "@near-wallet-selector/ledger";
// import { setupNightly } from "@near-wallet-selector/nightly";
import { CONTRACT_ID, METAPOOL_CONTRACT_ID, NETWORK_ID } from "../lib/near"
import { getConfig } from "../config";
import { setupWalletConnect } from "@near-wallet-selector/wallet-connect";
declare global {
  interface Window {
    selector: WalletSelector;
    modal: WalletSelectorModal;
    account_id: string | null;
    wallet: Wallet | null;
  }
}

interface WalletSelectorContextValue {
  selector: WalletSelector;
  modal: WalletSelectorModal;
  accounts: Array<AccountState>;
  accountId: string | null;
}

const WalletSelectorContext =
  React.createContext<WalletSelectorContextValue | null>(null);

export const WalletSelectorContextProvider: React.FC = ({ children }) => {
  const env = process.env.NEXT_PUBLIC_VERCEL_ENV || "development";
  const nearConfig = getConfig(env);
  const [selector, setSelector] = useState<WalletSelector | null>(null);
  const [modal, setModal] = useState<WalletSelectorModal | null>(null);
  const [accounts, setAccounts] = useState<Array<AccountState>>([]);

  const setupNearWalletCustom = () => {
    return async (options: any) => {
      const wallet = await setupMyNearWallet({
        walletUrl: nearConfig.walletUrl,
        iconUrl: "./assets/near-wallet-iconx.png",
      })(options);

      if (!wallet) {
        return null;
      }

      return {
        ...wallet,
        id: "near-wallet",
        metadata: {
          ...wallet.metadata,
          name: "NEAR Wallet",
          description: null,
          iconUrl: "./assets/near-wallet-icon.png",
          deprecated: false,
          available: true,
        },
      };
    };
  };

  const init = useCallback(async () => {
    const _selector = await setupWalletSelector({
      network: NETWORK_ID as NetworkId,
      debug: true,
      modules: [
        setupNearWalletCustom(),
        setupNarwallets(),
        // setupMyNearWallet(),
        // setupSender(),
        setupMathWallet(),
        setupNightly(),
        // setupLedger(),
        setupWalletConnect({
          projectId:
            process.env.NEXT_PUBLIC_WALLET_CONNECT_PROJECT_ID ||
            "3ec2226fd3f38b6fb82e789fcfc232bf",
          metadata: {
            name: "NEAR Wallet Selector for Meta Vote",
            description:
              "Wallet Connect integration on Wallet Selector for Meta Vote",
            url: "https://metavote.app/",
            icons: ["https://avatars.githubusercontent.com/u/37784886"],
          },
        }),
        /* setupNightlyConnect({
          url: "wss://ncproxy.nightly.app/app",
          appMetadata: {
            additionalInfo: "",
            application: "NEAR Wallet Selector",
            description: "Example dApp used by NEAR Wallet Selector",
            icon: "https://near.org/wp-content/uploads/2020/09/cropped-favicon-192x192.png",
          },
        }), */
      ],
    });


    const _modal = setupModal(_selector, { contractId: CONTRACT_ID || "" });
    const state = _selector.store.getState();
    setAccounts(state.accounts);

    window.selector = _selector;
    window.wallet = _selector.isSignedIn() ? await _selector.wallet() : null;
    window.modal = _modal;

    setSelector(_selector);
    setModal(_modal);
  }, []);

  useEffect(() => {
    init().catch((err) => {
      console.error(err);
      alert("Failed to initialise wallet selector");
    });
  }, [init]);

  useEffect(() => {
    if (!selector) {
      return;
    }

    const subscription = selector.store.observable
      .pipe(
        map((state) => state.accounts),
        distinctUntilChanged()
      )
      .subscribe((nextAccounts) => {
        window.account_id = (nextAccounts.find((account) => account.active)?.accountId) || null;
        setAccounts(nextAccounts);
      });

    return () => subscription.unsubscribe();
  }, [selector]);

  if (!selector || !modal) {
    return null;
  }

  const accountId =
    accounts.find((account) => account.active)?.accountId || null;

  return (
    <WalletSelectorContext.Provider
      value={{
        selector,
        modal,
        accounts,
        accountId,
      }}
    >
      {children}
    </WalletSelectorContext.Provider>
  );
};

export function useWalletSelector() {
  const context = useContext(WalletSelectorContext);

  if (!context) {
    throw new Error(
      "useWalletSelector must be used within a WalletSelectorContextProvider"
    );
  }

  return context;
}
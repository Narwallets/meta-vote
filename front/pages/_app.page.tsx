import "../styles/globals.css";
import type { AppProps } from "next/app";
import { ChakraProvider } from "@chakra-ui/react";
import "@fontsource/inter/variable.css";

import theme from "../theme/theme";
import { QueryClient, QueryClientProvider } from "react-query";
import Router, { useRouter } from "next/router";
import React, { useEffect, useState } from "react";
import * as gtag from "../lib/gtag";
import NProgress from "nprogress";
import NextHead from "next/head";

import "../styles/nprogress.css";
import Header from "./components/Header";
import Footer from "./components/Footer";
import Fonts from "./components/Fonts";
import { WalletSelectorContextProvider } from "../contexts/WalletSelectorContext";
import "@near-wallet-selector/modal-ui/styles.css";
import Script from "next/script";
import {
  PageBlocker,
  PageBlockerState,
} from "../components";
import { blockerStore } from "../stores/pageBlocker";

const isProduction = process.env.NEXT_PUBLIC_VERCEL_ENV == 'production';
const queryClient = new QueryClient();
function App({ Component, pageProps }: AppProps) {
  const router = useRouter();
  const [pageBlocerState, setPageBlockerState] = useState<PageBlockerState>({
    isActive: false,
    message: "",
  });
  useEffect(
    () =>
      blockerStore.subscribe((state) => {
        setPageBlockerState({
          isActive: state.isActive,
          message: state.message,
        });
      }),
    []
  );
  useEffect(() => {
    const handleRouteChange = (url: URL) => {
      /* invoke analytics function only for production */
      if (isProduction) {
        (window as any).gtag("config", gtag.GA_TRACKING_ID, {
          page_path: url,
        });
      }
    };
    router.events.on("routeChangeComplete", handleRouteChange);
    return () => {
      router.events.off("routeChangeComplete", handleRouteChange);
    };
  }, [router.events]);

  Router.events.on("routeChangeStart", (_) => NProgress.start());
  Router.events.on("routeChangeComplete", () => NProgress.done());
  Router.events.on("routeChangeError", () => NProgress.done());

  return (
    <ChakraProvider theme={theme}>
      <Fonts />
      <QueryClientProvider client={queryClient}>
        <WalletSelectorContextProvider>
          <NextHead>
            <meta charSet="UTF-8" />
            <title>
              {" "}
              Meta Vote - Allow any project to bootstrap liquidity through
              staking on Meta Pool.
            </title>
          </NextHead>
          <PageBlocker isActive={pageBlocerState.isActive} message={pageBlocerState.message} />
          <Header />
          <Component {...pageProps} />
          <Footer />
          {/* enable analytics script only for production */}
          {isProduction && (
            <>
              <Script
                src={`https://www.googletagmanager.com/gtag/js?id=${gtag.GA_TRACKING_ID}`}
                strategy="lazyOnload"
              />
              <Script id="google-analytics" strategy="lazyOnload">
                {`
                window.dataLayer = window.dataLayer || [];
                function gtag(){window.dataLayer.push(arguments);}
                gtag('js', new Date());

                gtag('config', '${gtag.GA_TRACKING_ID}');
              `}
              </Script>
            </>
          )}
        </WalletSelectorContextProvider>
      </QueryClientProvider>
    </ChakraProvider>
  );
}

export default App;

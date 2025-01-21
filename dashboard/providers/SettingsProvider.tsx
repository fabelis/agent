"use client";
import {
  useState,
  createContext,
  ReactNode,
  useContext,
  useEffect,
} from "react";

export interface SettingsContextProps {}

export const SettingsContext = createContext<SettingsContextProps | undefined>(
  undefined
);

export type Settings = {
  path_name: string;
  client_configs: any;
  enabled_clients: string[];
  completion_provider: string;
  embedding_provider: string;
  db: string;
};

export interface SettingsProviderProps {
  children: ReactNode;
}

export const SettingsProvider: React.FC<SettingsProviderProps> = ({
  children,
}) => {
  const contextValue = {};

  return (
    <SettingsContext.Provider value={contextValue}>
      {children}
    </SettingsContext.Provider>
  );
};

export const useSettings = (): SettingsContextProps => {
  const context = useContext(SettingsContext);
  if (!context) {
    throw new Error("useSettings must be used within a SettingsProvider");
  }
  return context;
};

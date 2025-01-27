"use client";
import {
  useState,
  createContext,
  ReactNode,
  useContext,
  useEffect,
} from "react";

export interface SettingsContextProps {
  settings: Settings[];
  selectedSettings: Settings | null;
  setSelectedSettings: (character: Settings) => void;
  saveSettings: (settings: Settings) => void;
  createSettings: (name: string) => void;
}

export const SettingsContext = createContext<SettingsContextProps | undefined>(
  undefined
);

export type Settings = {
  path_name: string;
  client_configs: ClientConfigs;
  enabled_clients: string[];
  completion_provider: string;
  embedding_provider: string;
  db: string;
};

export type ClientConfigs = {
  api: ApiConfig;
  discord: DiscordConfig;
  storytelling: StorytellingConfig;
  telegram: TelegramConfig;
  truth: TruthConfig;
  twitter: TwitterConfig;
};

export type ApiConfig = {
  port: number;
};

export type DiscordConfig = {
  surrounding_messages: number;
  selection_rate: number;
  debug: boolean;
};

export type StorytellingConfig = {
  port: number;
  paragraph_count: number[];
  use_tts: boolean;
};

export type TelegramConfig = {
  surrounding_messages: number;
  selection_rate: number;
  debug: boolean;
};

export type TruthConfig = {
  post_delay: number[];
  reply_delay: number[];
  search_delay: number;
  delay: number;
  debug: boolean;
};

export type TwitterConfig = {
  post_delay: number[];
  reply_delay: number[];
  search_delay: number;
  delay: number;
  debug: boolean;
};

export interface SettingsProviderProps {
  children: ReactNode;
}

export const SettingsProvider: React.FC<SettingsProviderProps> = ({
  children,
}) => {
  const [settings, setSettings] = useState<Settings[]>([]);
  const [selectedSettings, setSelectedSettings] = useState<Settings | null>(
    null
  );

  useEffect(() => {
    fetchSettings();
  }, []);

  useEffect(() => {
    if (settings.length > 0 && !selectedSettings) {
      setSelectedSettings(settings[0]);
    }
  }, [settings]);

  const fetchSettings = () => {
    fetch("/api/settings")
      .then((res) => {
        if (!res.ok) {
          throw new Error(`Failed to fetch settings: ${res.status}`);
        }
        return res.json();
      })
      .then((data) => {
        setSettings(data);
      })
      .catch((error) => {
        setSelectedSettings(null);
      });
  };

  const saveSettings = (settings: Settings) => {
    fetch("/api/settings/save", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(settings),
    })
      .then((res) => {
        if (!res.ok) {
          throw new Error(`Failed to save settings: ${res.status}`);
        }
        return res.json();
      })
      .then((data) => {
        setSelectedSettings(data);
        fetchSettings();
      })
      .catch((error) => {
        fetchSettings();
      });
  };

  const createSettings = (name: string) => {
    fetch("/api/settings/create", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        name,
      }),
    })
      .then((res) => {
        if (!res.ok) {
          throw new Error(`Failed to create settings: ${res.status}`);
        }
        return res.json();
      })
      .then((data) => {
        fetchSettings();
      })
      .catch((error) => {
        fetchSettings();
      });
  };

  const contextValue = {
    settings,
    selectedSettings,
    setSelectedSettings,
    saveSettings,
    createSettings,
  };

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

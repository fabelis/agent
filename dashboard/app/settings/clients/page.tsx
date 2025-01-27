"use client";

import React, { useState, useEffect } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Save } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { ClientConfigs, useSettings } from "@/providers/SettingsProvider";

export default function ClientConfigEditor() {
  const { selectedSettings, saveSettings } = useSettings();
  const [configs, setConfigs] = useState<ClientConfigs | null>(null);
  const [changedConfigs, setChangedConfigs] = useState<Set<string>>(new Set());

  useEffect(() => {
    if (selectedSettings) {
      setConfigs(selectedSettings.client_configs);
    }
  }, [selectedSettings]);

  const handleConfigChange = (configType: string, key: string, value: any) => {
    if (!configs) return;
    setConfigs((prev) => {
      if (!prev) return prev;
      return {
        ...prev,
        [configType]: {
          ...prev[configType as keyof ClientConfigs],
          [key]: value,
        },
      } as ClientConfigs;
    });
    setChangedConfigs((prev) => new Set(prev).add(configType));
  };

  const handleSave = (configType: string) => {
    if (!selectedSettings || !configs) return;

    const updatedSettings = {
      ...selectedSettings,
      client_configs: configs,
    };

    saveSettings(updatedSettings);
    setChangedConfigs((prev) => {
      const newSet = new Set(prev);
      newSet.delete(configType);
      return newSet;
    });
  };

  const renderConfigEditor = (configType: string, config: any) => {
    return (
      <Card key={configType} className="mb-6">
        <CardHeader>
          <CardTitle>
            {configType.charAt(0).toUpperCase() + configType.slice(1)} Config
          </CardTitle>
        </CardHeader>
        <CardContent>
          {Object.entries(config).map(([key, value]) => (
            <div key={key} className="mb-4">
              <Label htmlFor={`${configType}-${key}`} className="mb-2 block">
                {key}
              </Label>
              {key === "post_delay" || key === "reply_delay" ? (
                <div className="flex space-x-4">
                  <div className="flex-1">
                    <Label
                      htmlFor={`${configType}-${key}-min`}
                      className="text-sm text-muted-foreground"
                    >
                      Min
                    </Label>
                    <Input
                      id={`${configType}-${key}-min`}
                      type="number"
                      value={(value as number[])[0]}
                      onChange={(e) => {
                        const newValue = [...(value as number[])];
                        newValue[0] = Number(e.target.value);
                        handleConfigChange(configType, key, newValue);
                      }}
                    />
                  </div>
                  <div className="flex-1">
                    <Label
                      htmlFor={`${configType}-${key}-max`}
                      className="text-sm text-muted-foreground"
                    >
                      Max
                    </Label>
                    <Input
                      id={`${configType}-${key}-max`}
                      type="number"
                      value={(value as number[])[1]}
                      onChange={(e) => {
                        const newValue = [...(value as number[])];
                        newValue[1] = Number(e.target.value);
                        handleConfigChange(configType, key, newValue);
                      }}
                    />
                  </div>
                </div>
              ) : typeof value === "boolean" ? (
                <Switch
                  id={`${configType}-${key}`}
                  checked={value as boolean}
                  onCheckedChange={(checked) =>
                    handleConfigChange(configType, key, checked)
                  }
                />
              ) : Array.isArray(value) ? (
                <div className="flex space-x-4">
                  <div className="flex-1">
                    <Label
                      htmlFor={`${configType}-${key}-min`}
                      className="text-sm text-muted-foreground"
                    >
                      Min
                    </Label>
                    <Input
                      id={`${configType}-${key}-min`}
                      type="number"
                      value={(value as number[])[0]}
                      onChange={(e) => {
                        const newValue = [...(value as number[])];
                        newValue[0] = Number(e.target.value);
                        handleConfigChange(configType, key, newValue);
                      }}
                    />
                  </div>
                  <div className="flex-1">
                    <Label
                      htmlFor={`${configType}-${key}-max`}
                      className="text-sm text-muted-foreground"
                    >
                      Max
                    </Label>
                    <Input
                      id={`${configType}-${key}-max`}
                      type="number"
                      value={(value as number[])[1]}
                      onChange={(e) => {
                        const newValue = [...(value as number[])];
                        newValue[1] = Number(e.target.value);
                        handleConfigChange(configType, key, newValue);
                      }}
                    />
                  </div>
                </div>
              ) : (
                <Input
                  id={`${configType}-${key}`}
                  type={typeof value === "number" ? "number" : "text"}
                  value={value as string}
                  onChange={(e) =>
                    handleConfigChange(
                      configType,
                      key,
                      typeof value === "number"
                        ? Number(e.target.value)
                        : e.target.value
                    )
                  }
                />
              )}
            </div>
          ))}
          <Button
            onClick={() => handleSave(configType)}
            className="w-full mt-4"
            disabled={!changedConfigs.has(configType)}
          >
            <Save className="h-4 w-4 mr-2" />
            Save {configType.charAt(0).toUpperCase() + configType.slice(1)}{" "}
            Config
            {changedConfigs.has(configType) && (
              <Badge variant="secondary" className="ml-2">
                Unsaved Changes
              </Badge>
            )}
          </Button>
        </CardContent>
      </Card>
    );
  };

  return (
    <ScrollArea className="h-[calc(100vh-4rem)] pr-4">
      <div className="space-y-6">
        {configs &&
          Object.entries({
            api: configs.api || { port: 0 },
            discord: {
              ...{
                surrounding_messages: 0,
                selection_rate: 0,
                debug: false,
              },
              ...(configs.discord || {}),
            },
            storytelling: {
              ...{
                port: 0,
                paragraph_count: [0, 0],
                use_tts: false,
              },
              ...(configs.storytelling || {}),
            },
            telegram: {
              ...{
                surrounding_messages: 0,
                selection_rate: 0,
                debug: false,
              },
              ...(configs.telegram || {}),
            },
            truth: {
              ...{
                post_delay: [0, 0],
                reply_delay: [0, 0],
                search_delay: 0,
                delay: 0,
                debug: false,
              },
              ...(configs.truth || {}),
            },
            twitter: {
              ...{
                post_delay: [0, 0],
                reply_delay: [0, 0],
                search_delay: 0,
                delay: 0,
                debug: false,
              },
              ...(configs.twitter || {}),
            },
          }).map(([configType, config]) =>
            renderConfigEditor(configType, config)
          )}
      </div>
    </ScrollArea>
  );
}
